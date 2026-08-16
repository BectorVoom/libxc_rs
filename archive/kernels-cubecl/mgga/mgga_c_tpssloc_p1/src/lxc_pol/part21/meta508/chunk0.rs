//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2154/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2154<F: Float>(t5695: F, t912: F, t2842: F, t1557: F, t4395: F, t2792: F, t5730: F, t10661: F, t10756: F, t10828: F, t17192: F, t17451: F, t17454: F, t17471: F, t17490: F, t17493: F, t17496: F, t17500: F, t17504: F, t17506: F, t2905: F, t2930: F, t311: F) -> (F, F, F, F, F, F, F) {
    let t17507 = t5695 * t912;
    let t17509 = F::cast_from(6.0_f64) * t2842 * t17507;
    let t17510 = t1557 * t4395;
    let t17512 = F::cast_from(4.0_f64) * t2792 * t17510;
    let t17513 = t5730 * t912;
    let t17515 = F::cast_from(0.96491876992155210402e2_f64) * t10661 * t17513;
    let t17516 = -F::cast_from(0.10389515463408878255e3_f64) * t10828 * t17451 - F::cast_from(0.11696447245269292414e1_f64) * t2905 * t17454 - F::cast_from(0.310907e-1_f64) * t17471 * t311 + t17490 - F::cast_from(0.19751673498613801407e-1_f64) * t17192 + F::cast_from(0.17315859105681463759e2_f64) * t2930 * t17493 + F::cast_from(0.34631718211362927518e2_f64) * t2930 * t17496 + F::cast_from(0.10254018858216406658e4_f64) * t10756 * t17500 + t17504 - t17506 - t17509 + t17512 + t17515;
    (t17507, t17509, t17510, t17512, t17513, t17515, t17516)
}
