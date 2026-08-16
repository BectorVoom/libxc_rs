//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 215/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk215(t110: f64, t10: f64, t107: f64, t142: f64, t64: f64, t903: f64, t41: f64, t120: f64, t117: f64, t8: f64) -> (f64, f64, f64) {
    let t111 = t110 < -0.66725e-1_f64;
    let t911 = piecewise3(t111, 0.0_f64, 10.0_f64 / 9.0_f64 * t64 * t903 * t10 - 10.0_f64 / 27.0_f64 * t64 * t107 * t142);
    let t912 = t911 * t41;
    let t913 = t912 * t120;
    let t916 = t117 * t8;
    (t912, t913, t916)
}
