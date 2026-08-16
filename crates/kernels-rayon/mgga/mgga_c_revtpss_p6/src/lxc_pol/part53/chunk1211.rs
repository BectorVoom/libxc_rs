//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1211/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1211(t60221: f64, t8736: f64, t13272: f64, t32805: f64, t10301: f64, t34409: f64, t2247: f64, t29362: f64, t8435: f64, t60224: f64, t10309: f64, t122893: f64, t122911: f64, t122918: f64, t125238: f64, t125294: f64, t128368: f64, t128371: f64, t128374: f64, t128377: f64, t128444: f64, t32151: f64, t32586: f64, t32602: f64, t32795: f64, t32798: f64, t32802: f64, t32806: f64, t33621: f64, t34173: f64, t34402: f64, t34410: f64, t8623: f64, t8737: f64) -> f64 {
    let t129157 = t60221 * t8736;
    let t129160 = t13272 * t32805;
    let t129165 = t10301 * t34409;
    let t129169 = t2247 * t8435 * t29362;
    let t129180 = t60224 * t8736;
    let t129193 = t10309 * t34409;
    let t129196 = 5.0_f64 / 6.0_f64 * t122893 * t128368 + 5.0_f64 / 6.0_f64 * t122893 * t128371 - 5.0_f64 / 18.0_f64 * t32802 * t128374 - 5.0_f64 / 18.0_f64 * t32802 * t128377 + 5.0_f64 / 144.0_f64 * t129157 * t8623 + 5.0_f64 / 144.0_f64 * t129160 * t8623 + 5.0_f64 / 144.0_f64 * t34402 * t32151 + 5.0_f64 / 144.0_f64 * t129165 * t8623 + 5.0_f64 / 144.0_f64 * t129169 * t8623 + 5.0_f64 / 144.0_f64 * t34410 * t32151 + 5.0_f64 / 144.0_f64 * t32795 * t33621 + 5.0_f64 / 144.0_f64 * t32806 * t33621 + 5.0_f64 / 144.0_f64 * t8737 * t125238 - 5.0_f64 / 24.0_f64 * t129180 * t32586 + 5.0_f64 / 72.0_f64 * t34402 * t32602 - 5.0_f64 / 24.0_f64 * t122911 * t34173 - 5.0_f64 / 24.0_f64 * t122918 * t34173 - 5.0_f64 / 24.0_f64 * t32798 * t125294 - 5.0_f64 / 24.0_f64 * t32798 * t128444 - 5.0_f64 / 24.0_f64 * t129193 * t32586;
    t129196
}
