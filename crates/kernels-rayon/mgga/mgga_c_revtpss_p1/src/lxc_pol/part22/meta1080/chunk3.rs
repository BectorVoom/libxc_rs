//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3885/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3885(t1413: f64, t46835: f64, t74483: f64, t22061: f64, t9793: f64, t9794: f64, t22093: f64, t9962: f64, t13845: f64, t73731: f64, t9818: f64, t9835: f64) -> (f64, f64, f64, f64) {
    let t74638 = t46835 * t1413 * t74483;
    let t74641 = t9793 * t9794 * t22061;
    let t74656 = t9962 * t22093;
    let t74660 = t13845 * t9818 * t73731 * t9835;
    (t74638, t74641, t74656, t74660)
}
