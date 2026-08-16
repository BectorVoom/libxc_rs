//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2092/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2092(t2435: f64, t27965: f64, t14090: f64, t26054: f64, t14268: f64, t2022: f64, t7295: f64, t7296: f64, t7921: f64, t94608: f64, t94610: f64, t94613: f64, t94616: f64, t97792: f64, t97795: f64, t97798: f64, t97800: f64, t97804: f64, t97808: f64, t97810: f64, t97815: f64) -> f64 {
    let t97823 = t2435 * t27965;
    let t97825 = t26054 * t14090;
    let t97827 = 0.73171657588172351096e-2_f64 * t97792 + 0.65049603595885220126e-3_f64 * t97795 - t97798 - 0.22849835011101738147e-2_f64 * t97800 - t94608 - t97804 + 0.25702851531048074406e-1_f64 * t94613 + t97808 + 0.11565819519348392139e-2_f64 * t97810 + 0.23131639038696784278e-2_f64 * t94616 + 0.45699670022203476294e-2_f64 * t97815 + 0.8673628188205199462e0_f64 * t7295 * t7296 * t2022 * t14268 + 0.8673628188205199462e0_f64 * t94610 * t7921 - 0.73171657588172351096e-2_f64 * t97823 + 0.13009920719177044025e-1_f64 * t97825;
    t97827
}
