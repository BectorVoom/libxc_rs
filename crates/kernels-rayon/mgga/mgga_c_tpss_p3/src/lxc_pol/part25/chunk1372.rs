//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1372/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1372(t13603: f64, t1692: f64, t1812: f64, t18728: f64, t18807: f64, t20018: f64, t20048: f64, t20510: f64, t21485: f64, t21510: f64, t21513: f64, t21710: f64, t2439: f64, t3552: f64, t36547: f64, t5059: f64, t5678: f64, t5849: f64, t5853: f64, t6207: f64, t62829: f64, t66317: f64, t70813: f64, t70861: f64, t70872: f64, t70893: f64, t72188: f64, t72265: f64) -> f64 {
    let t72531 = t1692 * t1812 * t13603 / 2.0_f64 - t1692 * t5853 * t70861 / 2.0_f64 + t1692 * t62829 * t21510 + 2.0_f64 * t72188 * t20048 + 3.0_f64 * t3552 * t5849 * t21485 - t1692 * t72265 * t5678 / 2.0_f64 + 3.0_f64 * t2439 * t20510 * t6207 + 3.0_f64 * t18728 * t70893 - 3.0_f64 * t66317 * t20018 - 3.0_f64 / 2.0_f64 * t18728 * t70813 - 3.0_f64 * t18728 * t70872 + 3.0_f64 * t36547 * t21710 - t1692 * t18807 * t21513 + t1692 * t5849 * t5059 / 2.0_f64;
    t72531
}
