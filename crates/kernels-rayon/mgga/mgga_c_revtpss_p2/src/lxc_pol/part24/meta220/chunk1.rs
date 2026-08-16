//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 970/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk970(t11940: f64, t366: f64, t2434: f64, t371: f64, t373: f64, t367: f64, t1065: f64, t675: f64, t1035: f64, t11239: f64, t342: f64, t3145: f64, t334: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11941 = t11940 * t366;
    let t11970 = t371 * t2434 * t373;
    let t11972 = 0.63517063878621832551e-4_f64 * t367 * t11970;
    let t11986 = t675 * t1065;
    let t12046 = t11239 * t1035;
    let t12047 = t342 * t12046;
    let t12050 = 1.0_f64 / t3145 / t334;
    (t11941, t11970, t11972, t11986, t12046, t12047, t12050)
}
