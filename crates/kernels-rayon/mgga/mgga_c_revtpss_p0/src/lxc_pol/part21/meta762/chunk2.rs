//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2704/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2704(t2349: f64, t656: f64, t10227: f64, t97: f64, t10241: f64, t105: f64, t4273: f64, t588: f64, t10228: f64, t10242: f64, t13472: f64, t13475: f64, t13476: f64, t13485: f64, t13496: f64, t1504: f64, t1509: f64, t2255: f64, t2256: f64, t2350: f64, t2358: f64, t2362: f64, t31283: f64, t31443: f64, t46196: f64, t46212: f64, t580: f64, t658: f64, t661: f64, t9342: f64) -> f64 {
    let t49774 = t656 * t2349;
    let t49777 = t97 * t10227;
    let t49787 = t105 * t10241;
    let t49804 = 20.0_f64 * t97 * t4273 * t588;
    let t49809 = 50.0_f64 / 27.0_f64 * t656 * t13472 + 25.0_f64 * t656 * t13485 - 10.0_f64 / 3.0_f64 * t13496 * t2255 * t2362 - 10.0_f64 * t13475 * t9342 * t658 + 10.0_f64 * t13496 * t9342 * t661 - 100.0_f64 / 9.0_f64 * t49774 * t13476 - 10.0_f64 / 9.0_f64 * t49777 * t31283 * t2256 - 10.0_f64 / 9.0_f64 * t49777 * t2255 * t2350 + 10.0_f64 / 3.0_f64 * t13475 * t2255 * t2256 - 10.0_f64 / 9.0_f64 * t49787 * t31443 * t2362 + 10.0_f64 / 9.0_f64 * t49787 * t2255 * t2358 + 40.0_f64 / 81.0_f64 * t97 * t46196 * t1504 * t10228 + 10.0_f64 / 3.0_f64 * t97 * t2349 * t580 * t658 + t49804 + 40.0_f64 / 81.0_f64 * t105 * t46212 * t1509 * t10242;
    t49809
}
