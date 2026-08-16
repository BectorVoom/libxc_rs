//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1204/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1204(t125362: f64, t1937: f64, t125365: f64, t33602: f64, t6993: f64, t2042: f64, t28246: f64, t1916: f64, t32369: f64, t2040: f64, t28277: f64, t28264: f64, t572: f64, t7741: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t127401 = t125362 * t1937;
    let t127403 = t125365 * t1937;
    let t127405 = t33602 * t6993;
    let t127439 = t28246 * t2042;
    let t127442 = 12.0_f64 * t1916 * t32369;
    let t127443 = t2040 * t28277;
    let t127447 = 12.0_f64 * t572 * t28264 * t7741;
    (t127401, t127403, t127405, t127439, t127442, t127443, t127447)
}
