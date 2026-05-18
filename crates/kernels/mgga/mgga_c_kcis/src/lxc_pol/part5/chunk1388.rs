//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1388/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1388<F: Float>(t1354: F, t7475: F, t7472: F, t3947: F, t7019: F, t5613: F, t6117: F, t11539: F, t7002: F, t12730: F, t12744: F, t12751: F, t17784: F, t17895: F, t22843: F, t22847: F, t22850: F, t22854: F, t22861: F, t22864: F, t4331: F, t4356: F, t4366: F, t4373: F, t6111: F, t6118: F) -> F {
    let t22867 = t7475 * t1354;
    let t22870 = t7472 * t1354;
    let t22873 = t7019 * t3947;
    let t22874 = t22873 * t1354;
    let t22877 = t6117 * t5613;
    let t22880 = t7002 * t11539;
    let t22881 = t22880 * t1354;
    let t22884 = -F::new(2.0) * t4331 * t22843 + F::new(0.32164683177870697974e2) * t4356 * t22847 + F::new(0.64329366355741395948e2) * t4356 * t22850 + F::new(0.20691336878655965246e4) * t12730 * t22854 - F::new(0.23392893589820816284e1) * t17895 * t6111 + F::new(0.346315117987517266e2) * t17784 * t6118 + F::new(0.35089340384731224426e1) * t4373 * t22861 - F::new(0.23392893589820816284e1) * t4366 * t22864 - F::new(0.1038945353962551798e3) * t12744 * t22867 - F::new(0.11696446794910408142e1) * t4366 * t22870 + F::new(0.17315755899375863299e2) * t4373 * t22874 + F::new(0.34631511798751726598e2) * t4373 * t22877 + F::new(0.1025389702100779493e4) * t12751 * t22881;
    t22884
}
