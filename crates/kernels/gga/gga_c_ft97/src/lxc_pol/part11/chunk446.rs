//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 446/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk446<F: Float>(t200: F, t2417: F, t680: F, t2379: F, t2395: F, t235: F, t693: F, t226: F, t709: F) -> (F, F, F, F, F, F) {
    let t2418 = t2417 * t200;
    let t2419 = t680 * t2418;
    let t2422 = t2379 * t2395;
    let t2426 = F::cast_from(1.0_f64) / t693 / t235;
    let t2427 = t226 * t2426;
    let t2428 = t709 * t709;
    (t2418, t2419, t2422, t2426, t2427, t2428)
}
