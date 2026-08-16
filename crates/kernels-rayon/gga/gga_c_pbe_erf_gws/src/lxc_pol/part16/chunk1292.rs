//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1292/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1292(t14031: f64, t9382: f64, t9552: f64, t4028: f64, t9116: f64, t4142: f64, t51529: f64, t13953: f64, t14648: f64, t13796: f64, t14724: f64, t2352: f64, t343: f64, t3989: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54406 = t14031 * t9382;
    let t54408 = t14031 * t9552;
    let t54411 = t4028 * t9116;
    let t54427 = t51529 * t4142;
    let t54429 = t13953 * t14648;
    let t54461 = t3989 * t13796 * t14724 * t343 * t2352;
    (t54406, t54408, t54411, t54427, t54429, t54461)
}
