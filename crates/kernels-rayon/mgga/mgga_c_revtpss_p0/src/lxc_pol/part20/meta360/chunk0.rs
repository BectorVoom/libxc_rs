//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1309/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1309(t10073: f64, t10934: f64, t253: f64, t39552: f64, t2783: f64, t9646: f64, t22: f64, t251: f64, t837: f64, t2722: f64, t860: f64, t231: f64, t2782: f64) -> (f64, f64, f64, f64, f64) {
    let t39694 = t10073 * t10934;
    let t39697 = 0.88356352675825229576e-3_f64 * t39552 * t253;
    let t39698 = t9646 * t2783;
    let t39701 = t39698 * t251 * t22 * t837;
    let t39704 = t860 * t2722;
    let t39707 = t2782 * t2783 * t39704 * t231;
    (t39694, t39697, t39701, t39704, t39707)
}
