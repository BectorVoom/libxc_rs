//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1488/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1488(t1868: f64, t1883: f64, t46825: f64, t9793: f64, t22126: f64, t2689: f64, t22130: f64, t22056: f64, t9765: f64, t22021: f64, t808: f64, t9845: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74483 = t1883 * t1868;
    let t74485 = t9793 * t46825 * t74483;
    let t74491 = t2689 * t22126;
    let t74493 = t2689 * t22130;
    let t74511 = t9765 * t22056;
    let t74522 = t9845 * t808 * t22021;
    (t74483, t74485, t74491, t74493, t74511, t74522)
}
