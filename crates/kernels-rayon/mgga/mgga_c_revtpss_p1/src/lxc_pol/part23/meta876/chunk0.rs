//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2780/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2780(t22026: f64, t46802: f64, t9794: f64, t46694: f64, t6850: f64, t22294: f64, t48823: f64, t9816: f64, t1398: f64, t6843: f64, t22245: f64, t808: f64, t9736: f64) -> (f64, f64, f64, f64, f64) {
    let t74677 = t46802 * t9794 * t22026;
    let t74682 = t46694 * t6850;
    let t74698 = t9816 * t48823 * t22294;
    let t74700 = t6843 * t1398;
    let t74711 = t9736 * t808 * t22245;
    (t74677, t74682, t74698, t74700, t74711)
}
