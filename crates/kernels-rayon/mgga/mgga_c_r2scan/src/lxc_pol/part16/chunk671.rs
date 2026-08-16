//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 671/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk671(t4973: f64, t735: f64, t1422: f64, t425: f64, t1510: f64, t410: f64, t4911: f64, t89: f64, t36: f64, t409: f64, t1385: f64, t732: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4974 = t735 * t4973;
    let t4975 = 0.21687162600603479684e-1_f64 * t4974;
    let t4976 = t1422 * t425;
    let t4978 = t410 * t1510;
    let t4979 = 12.0_f64 * t4978;
    let t4980 = t4911 * t89;
    let t4981 = 24.0_f64 * t4980;
    let t4982 = t36 * t409;
    let t4983 = t4982 * t89;
    let t4987 = t732 * t1385;
    (t4975, t4976, t4979, t4981, t4982, t4983, t4987)
}
