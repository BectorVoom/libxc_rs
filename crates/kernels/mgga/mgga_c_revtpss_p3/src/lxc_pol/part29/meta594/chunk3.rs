//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1992/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1992<F: Float>(t28925: F, t531: F, t101435: F, t102070: F, t102111: F, t102148: F, t102175: F, t102222: F, t102248: F, t102282: F, t102313: F, t102341: F, t102374: F, t102406: F, t102443: F, t102584: F, t102612: F, t102642: F, t102669: F, t102700: F, t102738: F, t102764: F, t13625: F, t13872: F, t1450: F, t1453: F, t2014: F, t2108: F, t25082: F, t25802: F, t26218: F, t26399: F, t26411: F, t28167: F, t28176: F, t28196: F, t28286: F, t28686: F, t28707: F, t28718: F, t28727: F, t28927: F, t28929: F, t28939: F, t34495: F, t4248: F, t4297: F, t532: F, t569: F, t7235: F, t7238: F, t75365: F, t8108: F, t9069: F, t95088: F, t98496: F, t98579: F) -> F {
    let t102769 = t531 * t28925;
    let t102791 = -t2014 * t8108 * t25802 - F::new(4.0) * t26399 * t4297 + F::new(12.0) * t98579 * t28929 - F::new(2.0) * t4248 * t26218 + F::new(6.0) * t7235 * t28939 - F::new(6.0) * t28196 * t102070 * t98496 + t101435 * t2108 + F::new(2.0) * t7235 * t28927 + t2014 * t532 * (t102111 + t102148 + t102175 + t102222 + t102248 + t102282 + t102313 + t102341 + t102374 + t102406 + t102443 + t102584 + t102612 + t102642 + t102669 + t102700) * t1450 + (t102738 + t102764) * t569 - F::new(2.0) * t7235 * t28707 + F::new(6.0) * t2014 * t102769 * t7238 - F::new(6.0) * t25082 * t34495 * t13625 + F::new(6.0) * t28167 * t9069 * t13872 + F::new(2.0) * t28686 * t1453 + F::new(6.0) * t2014 * t26411 * t28176 - F::new(2.0) * t7235 * t28727 - F::new(6.0) * t95088 * t28718 + F::new(4.0) * t28196 * t28286 * t75365;
    t102791
}
