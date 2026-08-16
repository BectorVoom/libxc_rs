//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 408/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk408(t110: f64, t10: f64, t107: f64, t119: f64, t142: f64, t3020: f64, t64: f64, t903: f64, t41: f64, t120: f64, t912: f64, t919: f64, t212: f64, t9: f64) -> (f64, f64, f64, f64) {
    let t111 = t110 < -0.66725e-1_f64;
    let t3031 = piecewise3(t111, 0.0_f64, 10.0_f64 / 9.0_f64 * t64 * t3020 * t10 - 20.0_f64 / 27.0_f64 * t64 * t903 * t142 + 40.0_f64 / 81.0_f64 * t64 * t107 * t119);
    let t3032 = t3031 * t41;
    let t3033 = t3032 * t120;
    let t3036 = t912 * t919;
    let t3042 = 1.0_f64 / t9 / t212;
    (t3032, t3033, t3036, t3042)
}
