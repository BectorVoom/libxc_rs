//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1487/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1487(t22021: f64, t9793: f64, t9794: f64, t6876: f64, t9909: f64, t22026: f64, t46929: f64, t808: f64, t22259: f64, t9976: f64, t22125: f64, t2713: f64, t3964: f64) -> (f64, f64, f64, f64, f64) {
    let t74341 = t9793 * t9794 * t22021;
    let t74358 = t9909 * t6876;
    let t74362 = t46929 * t808 * t22026;
    let t74429 = t9976 * t22259;
    let t74437 = t3964 * t2713 * t22125;
    (t74341, t74358, t74362, t74429, t74437)
}
