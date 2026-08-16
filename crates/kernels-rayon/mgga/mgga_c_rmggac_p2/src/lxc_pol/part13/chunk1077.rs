//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1077/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1077(t40560: f64, t40562: f64, t40578: f64, t275: f64, t9677: f64, t1550: f64, t2211: f64, t27111: f64, t35795: f64, t37860: f64, t37866: f64, t4041: f64, t40564: f64, t40566: f64, t40568: f64, t40573: f64, t40607: f64, t40610: f64, t40614: f64, t40619: f64, t5016: f64, t9315: f64, t9370: f64) -> f64 {
    let t43466 = 0.1489760996265424379e-3_f64 * t40560;
    let t43467 = 0.1489760996265424379e-3_f64 * t40562;
    let t43472 = 0.15965655602485078085e0_f64 * t40578;
    let t43481 = 2.0_f64 * t275 * t9677;
    let t43488 = t43466 - t43467 - 0.49658699875514145966e-4_f64 * t40564 + 0.49658699875514145966e-4_f64 * t40566 + 0.212822999466489197e-4_f64 * t40568 + 0.212822999466489197e-4_f64 * t40573 - t43472 - 0.23948483403727617128e0_f64 * t5016 * t9315 + 0.23948483403727617128e0_f64 * t1550 * t2211 * t27111 + 0.15965655602485078085e0_f64 * t35795 + t37860 + 0.20455996240684006298e-1_f64 * t40607 + t43481 + 0.11974241701863808564e0_f64 * t4041 * t9370 - 0.5987120850931904282e-1_f64 * t40610 + 0.20455996240684006298e-1_f64 * t40614 - 0.4726e1_f64 * t37866 - 0.638468998399467591e-4_f64 * t40619;
    t43488
}
