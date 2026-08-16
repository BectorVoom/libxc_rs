//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1326/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1326(t1679: f64, t3486: f64, t1290: f64, t7682: f64, t1981: f64, t3426: f64, t3432: f64, t7690: f64, t1982: f64, t6076: f64, t77: f64, t10292: f64, t582: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t65165 = t1679 * t3486;
    let t65169 = t7682 * t1290;
    let t65172 = t1981 * t3426;
    let t65175 = t1981 * t3432;
    let t65178 = t7690 * t1290;
    let t65182 = t77 * t6076 * t1982;
    let t65189 = t10292 * t582;
    (t65165, t65169, t65172, t65175, t65178, t65182, t65189)
}
