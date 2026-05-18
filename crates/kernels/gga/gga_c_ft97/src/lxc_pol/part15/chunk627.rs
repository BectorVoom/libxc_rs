//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 627/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk627<F: Float>(t1240: F, t2842: F, t1250: F, t8232: F, t2770: F, t848: F, t319: F, t871: F, t10478: F, t2766: F, t10491: F, t2843: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15128 = t1240 * t2842;
    let t15147 = t8232 * t1250;
    let t15191 = t2770 * t1240;
    let t15195 = t848 * t1240;
    let t15229 = t2770 * t319;
    let t15254 = t848 * t871;
    let t15290 = t10478 * t319;
    let t15294 = t2766 * t871;
    let t15299 = t10491 * t319;
    let t15312 = t848 * t2843;
    (t15128, t15147, t15191, t15195, t15229, t15254, t15290, t15294, t15299, t15312)
}
