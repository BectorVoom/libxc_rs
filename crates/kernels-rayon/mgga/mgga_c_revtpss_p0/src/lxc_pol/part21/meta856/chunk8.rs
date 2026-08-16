//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3255/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3255(t117: f64, t60498: f64, t10259: f64, t93: f64, t10416: f64, t1312: f64, t13426: f64, t13435: f64, t13440: f64, t13514: f64, t1518: f64, t18227: f64, t2322: f64, t2371: f64, t4248: f64, t4292: f64, t46126: f64, t49686: f64, t49693: f64, t49830: f64, t49851: f64, t5523: f64, t60206: f64, t670: f64) -> (f64, f64) {
    let t60499 = t60498 * t117;
    let t60551 = t93 * t10259;
    let t60556 = 2.0_f64 * t10259 * t4248 + 6.0_f64 * t10416 * t4292 + 2.0_f64 * t1312 * t49830 + 6.0_f64 * t13426 * t2371 + 12.0_f64 * t13435 * t4292 + 6.0_f64 * t13440 * t4292 + 6.0_f64 * t13514 * t2322 + 6.0_f64 * t13514 * t5523 + 2.0_f64 * t1518 * t46126 + 6.0_f64 * t1518 * t49693 + 6.0_f64 * t1518 * t49851 + 2.0_f64 * t1518 * t60551 + 6.0_f64 * t18227 * t2371 + 6.0_f64 * t49686 * t670 + 6.0_f64 * t60206 + t60499;
    (t60499, t60556)
}
