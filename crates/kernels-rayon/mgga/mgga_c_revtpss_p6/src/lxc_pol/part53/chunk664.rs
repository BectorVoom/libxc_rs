//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 664/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk664(t239: f64, t7036: f64, t820: f64, t839: f64, t1946: f64, t846: f64, t233: f64, t64: f64, t857: f64, t7024: f64, t7026: f64, t7032: f64, t7035: f64) -> (f64, f64, f64, f64, f64) {
    let t7038 = t820 * t7036 * t239;
    let t7039 = t7038 * t839;
    let t7041 = t1946 * t846;
    let t7042 = 0.20007875121765877254e-2_f64 * t7041;
    let t7043 = t233 * t64;
    let t7045 = t820 * t7043 * t239;
    let t7046 = t7045 * t857;
    let t7048 = -t7024 - t7026 / 48.0_f64 - t7032 + t7035 - 0.42874018118069736972e-3_f64 * t7039 - t7042 - 0.17149607247227894789e-2_f64 * t7046;
    (t7038, t7042, t7043, t7045, t7048)
}
