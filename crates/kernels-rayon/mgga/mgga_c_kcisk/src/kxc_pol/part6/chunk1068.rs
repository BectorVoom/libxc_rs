//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1068/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1068(t14784: f64, t14785: f64, t19543: f64, t30592: f64, t30595: f64, t30599: f64, t30603: f64, t30613: f64, t30617: f64, t30623: f64, t30626: f64, t30629: f64, t30632: f64, t30635: f64) -> f64 {
    let t31556 = 0.94674375e0_f64 * t30613 - t14784 - t14785 - 0.34731666666666666667e0_f64 * t19543 + 0.264729375e1_f64 * t30617 + 0.20659e1_f64 * t30595 - 0.309885e1_f64 * t30599 - 0.57386111111111111112e0_f64 * t30592 - 0.516475e0_f64 * t30603 - 0.157790625e0_f64 * t30623 - 0.46308888888888888889e-1_f64 * t30626 - 0.104195e0_f64 * t30629 + 0.20839e0_f64 * t30632 - 0.62517e0_f64 * t30635;
    t31556
}
