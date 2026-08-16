//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2489/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2489(t14224: f64, t49068: f64, t9793: f64, t13800: f64, t46670: f64, t3964: f64, t5617: f64, t9732: f64, t136: f64, t216: f64, t9747: f64, t14230: f64, t46802: f64) -> (f64, f64, f64, f64, f64) {
    let t49070 = t9793 * t49068 * t14224;
    let t49071 = 0.13553694749236397037e-4_f64 * t49070;
    let t49087 = t46670 * t13800;
    let t49090 = t3964 * t9732 * t5617;
    let t49093 = t216 * t9747 * t136;
    let t49103 = t46802 * t49068 * t14230;
    (t49071, t49087, t49090, t49093, t49103)
}
