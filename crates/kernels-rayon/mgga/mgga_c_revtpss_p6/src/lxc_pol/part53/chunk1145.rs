//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1145/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1145(t121116: f64, t33930: f64, t33935: f64, t686: f64, t72: f64, t121338: f64, t121310: f64, t1389: f64, t32282: f64, t1882: f64, t561: f64, t247: f64, t31752: f64, t5675: f64) -> (f64, f64, f64, f64, f64) {
    let t125617 = t121116 * t33930;
    let t125620 = t33935 * t72 * t686;
    let t125621 = t121338 * t125620;
    let t125623 = t121310 * t125620;
    let t125625 = t32282 * t1389;
    let t125627 = t561 * t1882;
    let t125630 = t31752 * t125625 * t247 * t125627 * t5675;
    (t125617, t125621, t125623, t125627, t125630)
}
