//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1992/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1992(t28925: f64, t531: f64, t101435: f64, t102070: f64, t102111: f64, t102148: f64, t102175: f64, t102222: f64, t102248: f64, t102282: f64, t102313: f64, t102341: f64, t102374: f64, t102406: f64, t102443: f64, t102584: f64, t102612: f64, t102642: f64, t102669: f64, t102700: f64, t102738: f64, t102764: f64, t13625: f64, t13872: f64, t1450: f64, t1453: f64, t2014: f64, t2108: f64, t25082: f64, t25802: f64, t26218: f64, t26399: f64, t26411: f64, t28167: f64, t28176: f64, t28196: f64, t28286: f64, t28686: f64, t28707: f64, t28718: f64, t28727: f64, t28927: f64, t28929: f64, t28939: f64, t34495: f64, t4248: f64, t4297: f64, t532: f64, t569: f64, t7235: f64, t7238: f64, t75365: f64, t8108: f64, t9069: f64, t95088: f64, t98496: f64, t98579: f64) -> f64 {
    let t102769 = t531 * t28925;
    let t102791 = -t2014 * t8108 * t25802 - 4.0_f64 * t26399 * t4297 + 12.0_f64 * t98579 * t28929 - 2.0_f64 * t4248 * t26218 + 6.0_f64 * t7235 * t28939 - 6.0_f64 * t28196 * t102070 * t98496 + t101435 * t2108 + 2.0_f64 * t7235 * t28927 + t2014 * t532 * (t102111 + t102148 + t102175 + t102222 + t102248 + t102282 + t102313 + t102341 + t102374 + t102406 + t102443 + t102584 + t102612 + t102642 + t102669 + t102700) * t1450 + (t102738 + t102764) * t569 - 2.0_f64 * t7235 * t28707 + 6.0_f64 * t2014 * t102769 * t7238 - 6.0_f64 * t25082 * t34495 * t13625 + 6.0_f64 * t28167 * t9069 * t13872 + 2.0_f64 * t28686 * t1453 + 6.0_f64 * t2014 * t26411 * t28176 - 2.0_f64 * t7235 * t28727 - 6.0_f64 * t95088 * t28718 + 4.0_f64 * t28196 * t28286 * t75365;
    t102791
}
