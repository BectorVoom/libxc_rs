//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2046/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2046(t109118: f64, t111066: f64, t1310: f64, t13426: f64, t18227: f64, t2014: f64, t2055: f64, t2093: f64, t21658: f64, t22483: f64, t22506: f64, t2322: f64, t28737: f64, t28760: f64, t28939: f64, t29506: f64, t30138: f64, t30558: f64, t30563: f64, t30581: f64, t30589: f64, t30617: f64, t4248: f64, t4254: f64, t508: f64, t5920: f64, t651: f64, t7235: f64, t7374: f64, t7474: f64, t7488: f64, t7489: f64, t7536: f64, t7898: f64, t7978: f64) -> f64 {
    let t111130 = -2.0_f64 * t2322 * t30563 - 2.0_f64 * t4254 * t30563 - 2.0_f64 * t651 * t21658 * t2055 - t2014 * t7536 * t22483 + 2.0_f64 * t7235 * t30617 + 3.0_f64 * t2014 * t7488 * t109118 + t2093 * t22506 + 3.0_f64 * t29506 * t7489 - 4.0_f64 * t4248 * t28737 - 4.0_f64 * t30138 * t7374 - 4.0_f64 * t13426 * t7978 - 4.0_f64 * t18227 * t7978 - 4.0_f64 * t4248 * t28760 + 3.0_f64 * t7235 * t30581 - 2.0_f64 * t111066 * t508 - 2.0_f64 * t30589 * t1310 + 6.0_f64 * t7898 * t28939 - 2.0_f64 * t4254 * t30558 - 2.0_f64 * t651 * t7474 * t5920;
    t111130
}
