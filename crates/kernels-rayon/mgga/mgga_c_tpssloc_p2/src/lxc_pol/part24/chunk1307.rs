//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1307/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1307(t225: f64, t814: f64, t6648: f64, t81612: f64, t23021: f64, t6547: f64, t23151: f64, t2613: f64, t30684: f64, t4281: f64, t6660: f64, t808: f64, t81563: f64, t81568: f64, t81571: f64, t81575: f64, t81585: f64, t81589: f64, t81592: f64, t81595: f64, t81599: f64, t81600: f64, t81602: f64, t81606: f64, t81610: f64, t9632: f64) -> (f64, f64) {
    let t81613 = t225 * t814;
    let t81615 = t81612 * t81613 * t6648;
    let t81617 = t6547 * t23021;
    let t81621 = -0.9869604401089358619e-1_f64 * t81563 + 0.49348022005446793095e-1_f64 * t81568 - 0.12337005501361698274e-1_f64 * t81571 + 0.49348022005446793095e-1_f64 * t81575 + 3.0_f64 * t808 * t23151 + 6.0_f64 * t4281 * t30684 * t9632 - 0.14804406601634037928e0_f64 * t81585 + 0.49348022005446793095e-1_f64 * t81589 - 0.23029076935875170111e0_f64 * t81592 - 0.24674011002723396547e-1_f64 * t81595 - t81599 + 0.78134368175290755733e-1_f64 * t81600 + 0.19190897446562641759e0_f64 * t81602 + 0.9869604401089358619e-1_f64 * t81606 + 0.49348022005446793095e-1_f64 * t81610 + 0.24674011002723396547e-1_f64 * t81615 - 0.57572692339687925277e-1_f64 * t81617 + 3.0_f64 * t2613 * t6660;
    (t81613, t81621)
}
