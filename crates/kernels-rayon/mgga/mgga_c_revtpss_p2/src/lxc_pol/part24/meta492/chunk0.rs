//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1489/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1489(t6846: f64, t9909: f64, t1413: f64, t46835: f64, t74483: f64, t22061: f64, t9793: f64, t9794: f64, t22026: f64, t46802: f64, t46694: f64, t6850: f64) -> (f64, f64, f64, f64, f64) {
    let t74585 = t9909 * t6846;
    let t74638 = t46835 * t1413 * t74483;
    let t74641 = t9793 * t9794 * t22061;
    let t74677 = t46802 * t9794 * t22026;
    let t74682 = t46694 * t6850;
    (t74585, t74638, t74641, t74677, t74682)
}
