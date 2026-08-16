//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1359/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1359(t1052: f64, t1635: f64, t18074: f64, t1955: f64, t21662: f64, t21676: f64, t21691: f64, t25406: f64, t25757: f64, t25758: f64, t25778: f64, t28495: f64, t28713: f64, t3174: f64, t43604: f64, t4557: f64, t5920: f64, t5943: f64, t5944: f64, t6687: f64, t6704: f64, t6705: f64, t7600: f64, t7624: f64, t89617: f64, t99221: f64, t99877: f64) -> f64 {
    let t106492 = -0.82246703342411321825e-2_f64 * t6687 * t6704 * t6705 * t21662 + 0.82246703342411321826e-2_f64 * t99877 - 3.0_f64 * t25778 * t5944 - 3.0_f64 * t99221 * t1635 + 6.0_f64 * t18074 * t7600 + 6.0_f64 * t1052 * t3174 * t7624 * t5943 + 6.0_f64 * t4557 * t28713 + 6.0_f64 * t25778 * t5920 + 0.49348022005446793095e-1_f64 * t6687 * t25406 * t28495 - 0.54831135561607547884e-2_f64 * t89617 + 24.0_f64 * t1052 * t43604 * t1955 * t21676 - 18.0_f64 * t25757 * t25758 * t21691;
    t106492
}
