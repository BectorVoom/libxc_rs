//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1419/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1419(t22633: f64, t26421: f64, t6420: f64, t6976: f64, t107210: f64, t107367: f64, t107377: f64, t107381: f64, t107385: f64, t1336: f64, t20568: f64, t28171: f64, t5234: f64, t544: f64, t553: f64, t6987: f64, t81147: f64, t81154: f64, t90993: f64, t91000: f64, t97111: f64, t97124: f64, t97137: f64, t97142: f64, t97148: f64, t97161: f64) -> f64 {
    let t107389 = t22633 * t6976 * t26421 * t6420;
    let t107391 = -0.12337005501361698274e-1_f64 * t97111 + 6.0_f64 * t5234 * t28171 - 0.24674011002723396547e-1_f64 * t90993 - t1336 * t6987 * t20568 - 0.23029076935875170111e0_f64 * t97124 + 0.11514538467937585055e0_f64 * t97137 + 0.49348022005446793095e-1_f64 * t107367 + 0.12337005501361698274e-1_f64 * t97142 - 0.19190897446562641759e0_f64 * t91000 + 0.57572692339687925277e-1_f64 * t97148 - 0.74022033008170189643e-1_f64 * t97161 + t544 * t553 * t107210 - 0.16449340668482264365e-1_f64 * t107377 - 0.9869604401089358619e-1_f64 * t107381 - t81147 - 0.14804406601634037928e0_f64 * t107385 + 0.49348022005446793095e-1_f64 * t107389 + t81154;
    t107391
}
