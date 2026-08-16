//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1098/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1098(t5695: f64, t912: f64, t2842: f64, t1557: f64, t4395: f64, t2792: f64, t5730: f64, t10661: f64, t10756: f64, t10828: f64, t17192: f64, t17451: f64, t17454: f64, t17471: f64, t17490: f64, t17493: f64, t17496: f64, t17500: f64, t17504: f64, t17506: f64, t2905: f64, t2930: f64, t311: f64) -> (f64, f64, f64, f64) {
    let t17507 = t5695 * t912;
    let t17509 = 6.0_f64 * t2842 * t17507;
    let t17510 = t1557 * t4395;
    let t17512 = 4.0_f64 * t2792 * t17510;
    let t17513 = t5730 * t912;
    let t17515 = 0.96491876992155210402e2_f64 * t10661 * t17513;
    let t17516 = -0.10389515463408878255e3_f64 * t10828 * t17451 - 0.11696447245269292414e1_f64 * t2905 * t17454 - 0.310907e-1_f64 * t17471 * t311 + t17490 - 0.19751673498613801407e-1_f64 * t17192 + 0.17315859105681463759e2_f64 * t2930 * t17493 + 0.34631718211362927518e2_f64 * t2930 * t17496 + 0.10254018858216406658e4_f64 * t10756 * t17500 + t17504 - t17506 - t17509 + t17512 + t17515;
    (t17509, t17512, t17515, t17516)
}
