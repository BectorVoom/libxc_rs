//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1580/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1580(t20090: f64, t3117: f64, t1651: f64, t2857: f64, t4181: f64, t3092: f64, t2852: f64, t11703: f64, t19611: f64, t4910: f64, t11859: f64, t15850: f64, t16095: f64, t16165: f64, t16218: f64, t16220: f64, t1675: f64, t20075: f64, t20079: f64, t20083: f64, t3091: f64, t3115: f64, t4837: f64) -> (f64, f64, f64, f64, f64) {
    let t20091 = t3117 * t20090;
    let t20094 = t1651 * t2857;
    let t20095 = t20094 * t4181;
    let t20096 = t3092 * t20095;
    let t20099 = t1651 * t2852;
    let t20100 = t20099 * t4181;
    let t20101 = t11703 * t20100;
    let t20104 = t19611 * t4910;
    let t20105 = t3117 * t20104;
    let t20108 = -0.42874018118069736972e-3_f64 * t11859 * t20075 + t16165 + 0.14291339372689912324e-3_f64 * t3091 * t20079 + 0.42874018118069736972e-3_f64 * t4837 * t20083 + 0.28582678745379824648e-3_f64 * t15850 * t1675 + t16218 - t16220 / 648.0_f64 - 0.42874018118069736972e-3_f64 * t3115 * t20091 + 0.57165357490759649296e-3_f64 * t16095 * t20096 - 0.47637797908966374413e-3_f64 * t16095 * t20101 - 0.21437009059034868486e-3_f64 * t3115 * t20105;
    (t20091, t20096, t20101, t20105, t20108)
}
