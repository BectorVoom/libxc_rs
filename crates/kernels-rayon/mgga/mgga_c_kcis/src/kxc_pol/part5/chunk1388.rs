//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1388/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1388(t1354: f64, t7475: f64, t7472: f64, t3947: f64, t7019: f64, t5613: f64, t6117: f64, t11539: f64, t7002: f64, t12730: f64, t12744: f64, t12751: f64, t17784: f64, t17895: f64, t22843: f64, t22847: f64, t22850: f64, t22854: f64, t22861: f64, t22864: f64, t4331: f64, t4356: f64, t4366: f64, t4373: f64, t6111: f64, t6118: f64) -> f64 {
    let t22867 = t7475 * t1354;
    let t22870 = t7472 * t1354;
    let t22873 = t7019 * t3947;
    let t22874 = t22873 * t1354;
    let t22877 = t6117 * t5613;
    let t22880 = t7002 * t11539;
    let t22881 = t22880 * t1354;
    let t22884 = -2.0_f64 * t4331 * t22843 + 0.32164683177870697974e2_f64 * t4356 * t22847 + 0.64329366355741395948e2_f64 * t4356 * t22850 + 0.20691336878655965246e4_f64 * t12730 * t22854 - 0.23392893589820816284e1_f64 * t17895 * t6111 + 0.346315117987517266e2_f64 * t17784 * t6118 + 0.35089340384731224426e1_f64 * t4373 * t22861 - 0.23392893589820816284e1_f64 * t4366 * t22864 - 0.1038945353962551798e3_f64 * t12744 * t22867 - 0.11696446794910408142e1_f64 * t4366 * t22870 + 0.17315755899375863299e2_f64 * t4373 * t22874 + 0.34631511798751726598e2_f64 * t4373 * t22877 + 0.1025389702100779493e4_f64 * t12751 * t22881;
    t22884
}
