//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2490/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2490(t46888: f64, t48908: f64, t1413: f64, t46835: f64, t48694: f64, t13775: f64, t9793: f64, t9794: f64, t5690: f64, t9741: f64, t2659: f64, t5744: f64, t816: f64) -> (f64, f64, f64, f64, f64) {
    let t49105 = t46888 * t48908;
    let t49121 = t46835 * t1413 * t48694;
    let t49122 = 0.30492001685571196935e-4_f64 * t49121;
    let t49124 = t9793 * t9794 * t13775;
    let t49125 = 0.13553694749236397037e-4_f64 * t49124;
    let t49126 = t9741 * t5690;
    let t49127 = 35.0_f64 / 72.0_f64 * t49126;
    let t49137 = t816 * t2659 * t5744;
    (t49105, t49122, t49125, t49127, t49137)
}
