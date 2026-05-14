//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 403/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk403<F: Float>(t2999: F, t665: F, t1132: F, t375: F, t89: F, t1131: F, t2371: F, t223: F, t226: F) -> (F, F, F, F) {
    let t3704 = t2999 * t665;
    let t3710 = t89 * t375 * t1132;
    let t3717 = t2371 * t1131;
    let t3724 = t223 * t226;
    (t3704, t3710, t3717, t3724)
}
