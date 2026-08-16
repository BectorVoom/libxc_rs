//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1882/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1882(t2097: f64, t9646: f64, t9648: f64, t7515: f64, t94894: f64, t25899: f64, t96192: f64, t25875: f64, t96186: f64, t94398: f64, t3916: f64, t96191: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96230 = 0.19637199382202157274e-3_f64 * t9646 * t2097 * t9648;
    let t96232 = t94894 * t7515;
    let t96234 = t25899 * t96192;
    let t96236 = t25875 * t96186;
    let t96237 = t96236 * t94398;
    let t96239 = t96191 * t3916;
    (t96230, t96232, t96234, t96236, t96237, t96239)
}
