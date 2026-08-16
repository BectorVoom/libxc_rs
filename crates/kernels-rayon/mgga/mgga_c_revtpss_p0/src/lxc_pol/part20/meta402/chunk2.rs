//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1493/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1493(t1065: f64, t3133: f64, t372: f64, t1043: f64, t1045: f64, t11165: f64, t3181: f64, t11156: f64, t1011: f64, t1028: f64, t11637: f64, t11774: f64, t15700: f64, t15701: f64, t16012: f64, t16226: f64, t16229: f64, t41248: f64, t41263: f64, t42279: f64, t42282: f64, t42284: f64, t42288: f64, t42290: f64, t4786: f64, t4919: f64) -> (f64, f64, f64, f64) {
    let t42300 = t372 * t1065 * t3133;
    let t42309 = t372 * t1065 * t1043;
    let t42310 = t1045 * t11165;
    let t42315 = t372 * t3181 * t1043;
    let t42316 = t1045 * t11156;
    let t42320 = -0.85748036236139473944e-3_f64 * t42279 * t1028 + 0.18292914397043087775e-1_f64 * t42282 - 0.17149607247227894789e-2_f64 * t42284 - 0.57165357490759649296e-3_f64 * t42288 + 0.13719685797782315831e-1_f64 * t42290 * t1028 + 7.0_f64 / 108.0_f64 * t1011 * t16012 * t41248 - t1011 * t4919 * t41263 / 6.0_f64 + 0.34299214494455789578e-2_f64 * t16226 * t42300 * t16229 + 0.34299214494455789577e-2_f64 * t11774 * t15701 * t11637 * t4786 - 0.34299214494455789578e-2_f64 * t15700 * t42309 * t42310 + 0.28582678745379824648e-2_f64 * t15700 * t42315 * t42316;
    (t42300, t42309, t42315, t42320)
}
