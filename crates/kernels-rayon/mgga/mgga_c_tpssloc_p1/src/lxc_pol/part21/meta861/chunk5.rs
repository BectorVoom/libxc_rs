//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3127/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3127(t15402: f64, t18225: f64, t3447: f64, t11589: f64, t18427: f64, t18221: f64, t15376: f64, t15399: f64, t15403: f64, t18409: f64, t11593: f64, t15314: f64, t15332: f64, t15335: f64, t15395: f64, t15415: f64, t63415: f64) -> f64 {
    let t64696 = t3447 * t15402 * t18225;
    let t64699 = t3447 * t11589 * t18427;
    let t64702 = t3447 * t15402 * t18221;
    let t64711 = t15376 * t15399;
    let t64713 = t15376 * t15403;
    let t64718 = t3447 * t11589 * t18409;
    let t64725 = -0.74074074074074074073e-3_f64 * t64696 + 0.37037037037037037036e-3_f64 * t64699 - 0.11111111111111111111e-2_f64 * t64702 - 0.86419753086419753084e-3_f64 * t3447 * t15395 * t63415 + 0.88888888888888888884e-2_f64 * t15376 * t15335 + 0.29629629629629629628e-2_f64 * t15376 * t15332 - 0.98765432098765432094e-3_f64 * t64711 + 0.19753086419753086419e-2_f64 * t64713 - 0.14814814814814814814e-2_f64 * t15376 * t15415 + 0.18518518518518518518e-3_f64 * t64718 + 0.27777777777777777777e-3_f64 * t3447 * t11593 * t18409 - 0.29629629629629629628e-2_f64 * t15376 * t15314;
    t64725
}
