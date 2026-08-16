//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1787/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1787(t1222: f64, t1261: f64, t12855: f64, t13100: f64, t17475: f64, t21040: f64, t24228: f64, t24535: f64, t247: f64, t24792: f64, t3604: f64, t3625: f64, t3626: f64, t3720: f64, t44225: f64, t5312: f64, t5381: f64, t83392: f64, t83394: f64, t83435: f64, t89822: f64, t89826: f64, t89863: f64, t90042: f64, t90262: f64, t91012: f64) -> f64 {
    let t91173 = t1222 * t5312 * t89826 / 6.0_f64 - 7.0_f64 / 108.0_f64 * t1222 * t17475 * t89822 - 0.22866142996303859718e-2_f64 * t83392 - 0.2540682555144873302e-2_f64 * t5381 * t24535 - 0.76220476654346199062e-2_f64 * t1261 * t247 * t13100 * t89863 - 0.22866142996303859718e-2_f64 * t83394 - 0.25724410870841842184e-2_f64 * t12855 * t3720 * t90042 * t3604 - 0.11433071498151929859e-2_f64 * t83435 - 0.85748036236139473944e-3_f64 * t3625 * t3626 * t21040 * t91012 - 0.17149607247227894789e-2_f64 * t3625 * t3626 * t21040 * t90262 - 0.2540682555144873302e-2_f64 * t3625 * t44225 * t24228 * t24792;
    t91173
}
