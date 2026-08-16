//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1304/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1304(t54119: f64, t6680: f64, t2134: f64, t8996: f64, t14015: f64, t9522: f64, t1150: f64, t51200: f64, t14028: f64, t3295: f64, t4023: f64, t9172: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54120 = t54119 * t6680;
    let t54122 = t2134 * t8996;
    let t54124 = t14015 * t9522;
    let t54126 = t51200 * t1150;
    let t54128 = t14028 * t3295;
    let t54129 = 7.0_f64 / 576.0_f64 * t54128;
    let t54130 = t9172 * t4023;
    (t54120, t54122, t54124, t54126, t54129, t54130)
}
