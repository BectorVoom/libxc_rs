//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1678/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1678(t324: f64, t88462: f64, t88475: f64, t41499: f64, t41502: f64, t88031: f64, t11409: f64, t11450: f64, t11509: f64, t15413: f64, t1621: f64, t1622: f64, t1634: f64, t23754: f64, t23755: f64, t23761: f64, t2943: f64, t2968: f64, t3012: f64, t3014: f64, t41759: f64, t4647: f64, t6157: f64, t6173: f64, t6177: f64, t6190: f64, t6205: f64, t63979: f64, t78111: f64, t78165: f64, t88008: f64, t88351: f64, t88368: f64, t88432: f64, t88445: f64, t88448: f64, t88451: f64) -> (f64, f64, f64) {
    let t88477 = (t88462 + t88475) * t324;
    let t88481 = 0.24955700379505800916e5_f64 * t41499 * t88031 * t41502;
    let t88499 = t88368 - t88432 + 4.0_f64 * t4647 * t23755 + 0.23392894490538584828e1_f64 * t78111 * t1634 + 0.51947577317044391277e2_f64 * t3012 * t88351 * t3014 - 0.12304822629859687989e5_f64 * t41759 * t88008 * t11509 + t88445 - t88448 - t88451 - 0.19751673498613801407e-1_f64 * t88477 - t88481 - 8.0_f64 * t2943 * t1622 * t23754 - 0.11579025239058625248e4_f64 * t11409 * t6177 * t6173 + 0.12865583598954028054e3_f64 * t2968 * t78165 * t1621 + 0.12414243100625616072e5_f64 * t11450 * t63979 * t6157 - 0.14035736694323150897e2_f64 * t15413 * t23761 + 0.21053605041484726346e2_f64 * t3012 * t6190 * t6205;
    (t88477, t88481, t88499)
}
