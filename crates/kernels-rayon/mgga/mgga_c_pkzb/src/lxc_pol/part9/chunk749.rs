//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 749/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk749(t1736: f64, t5257: f64, t1692: f64, t581: f64, t582: f64, t1698: f64, t579: f64, t583: f64, t1702: f64, t1712: f64, t50: f64, t5217: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5258 = t5257 * t1736;
    let t5261 = t581 * t582 * t1692;
    let t5264 = t1698 * t579;
    let t5265 = t5264 * t583;
    let t5267 = t1702 * t1712;
    let t5269 = t50 * t5217;
    (t5258, t5261, t5264, t5265, t5267, t5269)
}
