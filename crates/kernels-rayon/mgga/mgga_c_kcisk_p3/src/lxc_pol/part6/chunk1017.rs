//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1017/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1017(t13091: f64, t13092: f64, t19543: f64, t30592: f64, t30595: f64, t30599: f64, t30603: f64, t30613: f64, t30617: f64, t30623: f64, t30626: f64, t30629: f64, t30632: f64, t30635: f64) -> f64 {
    let t30704 = 0.247573125e0_f64 * t30613 - t13091 - t13092 - 0.27595e0_f64 * t19543 + 0.19419375e1_f64 * t30617 + 0.12077e1_f64 * t30595 - 0.181155e1_f64 * t30599 - 0.33547222222222222222e0_f64 * t30592 - 0.301925e0_f64 * t30603 - 0.412621875e-1_f64 * t30623 - 0.36793333333333333333e-1_f64 * t30626 - 0.82785e-1_f64 * t30629 + 0.16557e0_f64 * t30632 - 0.49671e0_f64 * t30635;
    t30704
}
