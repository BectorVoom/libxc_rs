//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1356/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1356(t2242: f64, t4230: f64, t15027: f64, t9270: f64, t15089: f64, t4414: f64, t14924: f64, t1205: f64, t14258: f64, t14264: f64, t14952: f64, t19631: f64, t2182: f64, t2376: f64, t2408: f64, t2409: f64, t26654: f64, t3066: f64, t3067: f64, t3207: f64, t34963: f64, t4088: f64, t4207: f64, t4227: f64, t51979: f64, t52582: f64, t54629: f64, t54636: f64, t6781: f64, t8589: f64, t9688: f64) -> f64 {
    let t55904 = t2242 * t4230;
    let t55918 = 7.0_f64 / 72.0_f64 * t9270 * t15027;
    let t55936 = 7.0_f64 / 72.0_f64 * t4414 * t15089;
    let t55942 = 7.0_f64 / 72.0_f64 * t4414 * t14924;
    let t55945 = -35.0_f64 / 432.0_f64 * t55904 - t54629 / 12.0_f64 - t52582 + t3066 * t2409 * t3067 * t1205 * t9688 / 48.0_f64 - t3207 * t2409 * t2376 * t4227 * t2182 / 16.0_f64 - t55918 - t3066 * t2409 * t34963 * t14264 / 16.0_f64 + t2408 * t2409 * t19631 * t4207 / 48.0_f64 + t2408 * t2409 * t8589 * t14258 / 24.0_f64 + t2408 * t2409 * t6781 * t14952 / 24.0_f64 - t55936 + t2408 * t2409 * t26654 * t4088 / 24.0_f64 - t55942 - t54636 / 48.0_f64 + 7.0_f64 / 72.0_f64 * t51979;
    t55945
}
