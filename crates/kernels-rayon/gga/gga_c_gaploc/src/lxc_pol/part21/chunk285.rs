//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 285/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk285(t1176: f64, t367: f64, t374: f64, t365: f64, t366: f64, t1072: f64, t54: f64, t1076: f64, t1126: f64, t1131: f64, t1138: f64, t1153: f64, t1161: f64, t1165: f64, t1169: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1178 = t367 * t1176 * t374;
    let t1180 = 0.58482233974552040708e0_f64 * t365 * t1178;
    let t1181 = t366 * t366;
    let t1182 = 1.0_f64 / t1181;
    let t1183 = t1182 * t1072;
    let t1184 = t54 * t54;
    let t1185 = 1.0_f64 / t1184;
    let t1186 = t1183 * t1185;
    let t1188 = 0.17315755899375863299e2_f64 * t365 * t1186;
    let t1189 = -t1126 - t1131 - t1138 + t1153 + t1161 + t1165 + t1169 + t1076 - t1180 - t1188;
    (t1178, t1180, t1182, t1185, t1186, t1188, t1189)
}
