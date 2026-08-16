//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1367/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1367(t10627: f64, t10697: f64, t236: f64, t807: f64, t10689: f64, t237: f64, t247: f64, t10709: f64, t10744: f64, t808: f64, t10752: f64, t10905: f64) -> (f64, f64, f64, f64) {
    let t40503 = t807 * t236 * t10697 * t10627;
    let t40507 = 0.28974367305964659283e0_f64 * t237 * t10689 * t247;
    let t40509 = t10744 * t808 * t10709;
    let t40511 = t10905 * t10752;
    (t40503, t40507, t40509, t40511)
}
