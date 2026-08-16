//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2596/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2596(t1211: f64, t20703: f64, t1214: f64, t6744: f64, t1277: f64, t1294: f64, t6573: f64, t1774: f64, t5245: f64) -> (f64, f64, f64, f64) {
    let t20704 = t1211 * t20703;
    let t20709 = t6744 * t1214;
    let t20710 = t1277 * t20709;
    let t20714 = t1277 * t6573 * t1294;
    let t20721 = t1774 * t5245;
    (t20704, t20710, t20714, t20721)
}
