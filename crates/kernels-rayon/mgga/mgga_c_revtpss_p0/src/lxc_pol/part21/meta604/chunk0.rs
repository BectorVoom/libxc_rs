//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2334/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2334(t10960: f64, t2435: f64, t2482: f64, t39620: f64, t686: f64, t72: f64, t879: f64, t10073: f64, t10934: f64, t253: f64, t39552: f64, t2783: f64, t9646: f64) -> (f64, f64, f64, f64, f64) {
    let t39687 = t2435 * t10960;
    let t39692 = t2482 * t879 * t72 * t686 * t39620;
    let t39694 = t10073 * t10934;
    let t39697 = 0.88356352675825229576e-3_f64 * t39552 * t253;
    let t39698 = t9646 * t2783;
    (t39687, t39692, t39694, t39697, t39698)
}
