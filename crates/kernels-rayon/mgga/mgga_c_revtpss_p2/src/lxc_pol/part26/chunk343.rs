//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 343/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk343(t265: f64, t502: f64, t1277: f64, t1294: f64, t1204: f64, t1210: f64, t1215: f64, t1271: f64, t1274: f64, t460: f64, t495: f64, t498: f64, t1128: f64, t1153: f64, t1193: f64, t1195: f64, t1200: f64, t198: f64, t336: f64, t895: f64) -> (f64, f64, f64, f64) {
    let t503 = t265 < t502;
    let t1295 = t1277 * t1294;
    let t1298 = 0.65854491829355115987e0_f64 * t1204 * t495 - 0.65854491829355115987e0_f64 * t1210 * t1215 + 0.65854491829355115987e0_f64 * t460 * t1271 - 0.65854491829355115987e0_f64 * t1274 * t1295;
    let t1300 = 1.0_f64 / t498;
    let t1304 = piecewise3(t503, t1298 * t1300 * t198 * t336 - t1128 + t1153 + t1193 + t1195 - t1200, t895);
    (t1295, t1298, t1300, t1304)
}
