//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2493/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2493(t1398: f64, t14141: f64, t14143: f64, t2434: f64, t14155: f64, t1432: f64, t2470: f64, t3999: f64, t5710: f64, t10069: f64, t14225: f64, t10136: f64, t14114: f64) -> (f64, f64, f64, f64, f64) {
    let t49256 = t14141 * t14143 * t2434 * t1398;
    let t49273 = t1432 * t14155 * t2470;
    let t49274 = 0.39029762157531132076e-1_f64 * t49273;
    let t49276 = t3999 * t5710;
    let t49289 = t10069 * t14225;
    let t49290 = 0.21951497276451705329e-1_f64 * t49289;
    let t49321 = t14114 * t10136;
    (t49256, t49274, t49276, t49290, t49321)
}
