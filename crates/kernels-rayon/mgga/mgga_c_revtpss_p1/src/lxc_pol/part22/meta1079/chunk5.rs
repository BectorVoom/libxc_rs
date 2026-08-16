//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3875/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3875(t221: f64, t22287: f64, t3978: f64, t9921: f64, t22289: f64, t3989: f64, t1868: f64, t1883: f64, t46825: f64, t9793: f64, t1399: f64, t47274: f64, t6849: f64, t9816: f64) -> (f64, f64, f64, f64, f64) {
    let t74477 = t221 * t22287;
    let t74479 = t3978 * t9921 * t74477;
    let t74481 = t3989 * t22289;
    let t74483 = t1883 * t1868;
    let t74485 = t9793 * t46825 * t74483;
    let t74489 = t9816 * t47274 * t6849 * t1399;
    (t74479, t74481, t74483, t74485, t74489)
}
