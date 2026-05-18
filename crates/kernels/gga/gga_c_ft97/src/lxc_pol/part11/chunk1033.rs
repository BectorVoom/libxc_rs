//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1033/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1033<F: Float>(t235: F, t9680: F, t226: F, t2428: F, t2393: F, t688: F, t13521: F, t2455: F, t709: F, t9548: F, t2395: F, t2417: F) -> (F, F, F, F, F, F, F) {
    let t41547 = F::new(1.0) / t9680 / t235;
    let t41548 = t226 * t41547;
    let t41549 = t2428 * t2428;
    let t41557 = t2393 * t688;
    let t41561 = t13521 * t2455;
    let t41569 = t2455 * t2455;
    let t41573 = t9548 * t709;
    let t41577 = t2395 * t2417;
    (t41548, t41549, t41557, t41561, t41569, t41573, t41577)
}
