//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3256/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3256(t10259: f64, t10416: f64, t10426: f64, t1310: f64, t13207: f64, t13216: f64, t13425: f64, t13426: f64, t13517: f64, t13537: f64, t13544: f64, t1453: f64, t1518: f64, t18227: f64, t1843: f64, t1911: f64, t2322: f64, t2372: f64, t4248: f64, t4254: f64, t4297: f64, t508: f64, t569: f64, t60499: f64, t60556: f64, t651: f64) -> f64 {
    let t60558 = -2.0_f64 * t10259 * t1843 * t651 - 2.0_f64 * t13207 * t1518 * t651 - 6.0_f64 * t10416 * t4297 + t10426 * t1911 - 3.0_f64 * t1310 * t13425 - 6.0_f64 * t13216 * t4248 - 6.0_f64 * t13426 * t2372 + 3.0_f64 * t13517 * t1453 - 6.0_f64 * t13537 * t2322 - 6.0_f64 * t13537 * t4254 - 6.0_f64 * t13544 * t4254 - 6.0_f64 * t18227 * t2372 - t508 * t60499 + t569 * t60556;
    t60558
}
