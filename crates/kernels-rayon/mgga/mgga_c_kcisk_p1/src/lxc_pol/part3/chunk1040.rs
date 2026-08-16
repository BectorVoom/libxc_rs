//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1040/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1040(t110: f64, t15363: f64, t15401: f64, t67: f64, t10: f64, t107: f64, t119: f64, t142: f64, t3020: f64, t64: f64, t903: f64, t918: f64, t41: f64) -> f64 {
    let t111 = t110 < -0.66725e-1_f64;
    let t15403 = t67 * (t15363 + t15401);
    let t15417 = piecewise3(t111, 0.0_f64, 10.0_f64 / 9.0_f64 * t64 * t15403 * t10 - 10.0_f64 / 9.0_f64 * t64 * t3020 * t142 + 40.0_f64 / 27.0_f64 * t64 * t903 * t119 - 280.0_f64 / 243.0_f64 * t64 * t107 * t918);
    let t15418 = t15417 * t41;
    t15418
}
