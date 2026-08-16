//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1008/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1008<F: Float>(t1354: F, t7475: F, t7472: F, t3947: F, t7019: F, t5613: F, t6117: F, t11539: F, t7002: F, t12730: F, t12744: F, t12751: F, t17784: F, t17895: F, t22843: F, t22847: F, t22850: F, t22854: F, t22861: F, t22864: F, t4331: F, t4356: F, t4366: F, t4373: F, t6111: F, t6118: F) -> F {
    let t22867 = t7475 * t1354;
    let t22870 = t7472 * t1354;
    let t22873 = t7019 * t3947;
    let t22874 = t22873 * t1354;
    let t22877 = t6117 * t5613;
    let t22880 = t7002 * t11539;
    let t22881 = t22880 * t1354;
    let t22884 = -F::cast_from(2.0_f64) * t4331 * t22843 + F::cast_from(0.32164683177870697974e2_f64) * t4356 * t22847 + F::cast_from(0.64329366355741395948e2_f64) * t4356 * t22850 + F::cast_from(0.20691336878655965246e4_f64) * t12730 * t22854 - F::cast_from(0.23392893589820816284e1_f64) * t17895 * t6111 + F::cast_from(0.346315117987517266e2_f64) * t17784 * t6118 + F::cast_from(0.35089340384731224426e1_f64) * t4373 * t22861 - F::cast_from(0.23392893589820816284e1_f64) * t4366 * t22864 - F::cast_from(0.1038945353962551798e3_f64) * t12744 * t22867 - F::cast_from(0.11696446794910408142e1_f64) * t4366 * t22870 + F::cast_from(0.17315755899375863299e2_f64) * t4373 * t22874 + F::cast_from(0.34631511798751726598e2_f64) * t4373 * t22877 + F::cast_from(0.1025389702100779493e4_f64) * t12751 * t22881;
    t22884
}
