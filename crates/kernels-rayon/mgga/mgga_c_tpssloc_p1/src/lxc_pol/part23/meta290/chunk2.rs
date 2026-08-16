//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1006/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1006(t21180: f64, t894: f64, t901: f64, t1547: f64, t5698: f64, t10599: f64, t10595: f64, t13598: f64, t13642: f64, t17149: f64, t17165: f64, t17175: f64, t17286: f64, t17288: f64, t17290: f64, t21161: f64, t21168: f64) -> (f64, f64, f64, f64, f64) {
    let t21181 = t894 * t21180;
    let t21183 = t901 * t21180;
    let t21185 = t5698 * t1547;
    let t21186 = t10599 * t21185;
    let t21188 = t10595 * t21185;
    let t21193 = -0.34731666666666666667e0_f64 * t13642 + 0.62517e0_f64 * t21161 - 0.68863333333333333332e0_f64 * t13598 + 0.34431666666666666666e0_f64 * t17149 - 0.103295e1_f64 * t17165 + 0.51647499999999999999e0_f64 * t17175 - 0.104195e0_f64 * t21168 + 0.3529725e1_f64 * t21181 + 0.6311625e0_f64 * t21183 - 0.157790625e0_f64 * t21186 + 0.264729375e1_f64 * t21188 + 0.69463333333333333335e-1_f64 * t17286 - 0.41678000000000000001e0_f64 * t17288 + 0.20839e0_f64 * t17290;
    (t21181, t21183, t21186, t21188, t21193)
}
