//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1620/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1620(t135: f64, t5889: f64, t973: f64, t5893: f64, t5884: f64, t4593: f64, t4650: f64, t4582: f64, t5398: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17615 = t135 * t5889;
    let t17616 = t973 * t17615;
    let t17620 = t135 * t5893;
    let t17621 = t973 * t17620;
    let t17624 = t135 * t5884;
    let t17625 = t973 * t17624;
    let t17631 = t4593 * t4650;
    let t17632 = t4582 * t17631;
    let t17635 = t5398 * t607;
    (t17615, t17616, t17620, t17621, t17624, t17625, t17631, t17632, t17635)
}
