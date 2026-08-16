//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1367/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1367(t1398: f64, t14076: f64, t14256: f64, t14426: f64, t1692: f64, t1812: f64, t18807: f64, t198: f64, t19809: f64, t20417: f64, t20514: f64, t207: f64, t21262: f64, t21678: f64, t2439: f64, t3552: f64, t3610: f64, t36547: f64, t3683: f64, t3724: f64, t4706: f64, t4802: f64, t52639: f64, t5849: f64, t5853: f64, t6354: f64, t66281: f64, t70771: f64, t72172: f64, t72265: f64, t821: f64, t823: f64) -> f64 {
    let t72363 = -t1692 * t18807 * t4802 - 2.0_f64 * t1692 * t66281 * t1398 + 6.0_f64 * t3552 * t5849 * t4706 + 6.0_f64 * t36547 * t21678 - 2.0_f64 * t1692 * t20514 * t3724 + 6.0_f64 * t2439 * t6354 * t3610 + 12.0_f64 * t3552 * t6354 * t3683 - 12.0_f64 * t20417 * t70771 - t1692 * t72265 * t821 + t198 * t207 * t72172 * t823 - t1692 * t5853 * t14426 - 6.0_f64 * t2439 * t5853 * t52639 - 6.0_f64 * t2439 * t18807 * t21262 - 6.0_f64 * t2439 * t20514 * t14076 + 6.0_f64 * t3552 * t1812 * t14256 - 6.0_f64 * t2439 * t20514 * t19809;
    t72363
}
