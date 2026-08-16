//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 443/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk443<F: Float>(t3188: F, t3194: F, t3193: F, t103: F, t942: F, t379: F, t1902: F, t432: F, t920: F, t1903: F, t447: F, t986: F) -> (F, F, F, F, F, F, F) {
    let t3195 = t3194 * t3188;
    let t3196 = t3193 * t3195;
    let t3199 = t103 * t942;
    let t3200 = t3199 * t379;
    let t3201 = t1902 * t3200;
    let t3204 = t920 * t432;
    let t3205 = t1903 * t3204;
    let t3206 = t1902 * t3205;
    let t3210 = t447 * t986 * t379;
    (t3195, t3196, t3200, t3201, t3205, t3206, t3210)
}
