//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1368/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1368<F: Float>(t3393: F, t7373: F, t143: F, t21453: F, t7369: F, t12406: F, t4219: F, t6281: F, t7361: F, t7365: F, t1153: F, t12397: F, t12401: F, t17627: F, t22054: F, t22059: F, t22094: F, t22134: F, t22142: F, t22160: F, t22165: F, t22169: F, t4202: F, t545: F, t5947: F, t5958: F) -> F {
    let t22514 = t3393 * t7373;
    let t22520 = t21453 * t143;
    let t22528 = t3393 * t7369;
    let t22531 = t4219 * t12406 * t6281;
    let t22534 = t3393 * t7361;
    let t22536 = t3393 * t7365;
    let t22542 = -F::new(0.46434375e-2) * t5947 * t22165 + F::new(0.9286875e-2) * t5947 * t22169 - F::cast_from(0.17687407407407407407e-1_f64) * t22514 - F::new(0.9286875e-2) * t4202 * t22059 + F::new(0.1857375e-1) * t4202 * t22134 + F::new(0.619125e-2) * t22520 * t545 + F::new(0.46434375e-2) * t5947 * t22054 + F::new(0.24765e-1) * t5958 * t22142 + F::cast_from(0.88437037037037037037e-2_f64) * t12397 - t12401 + F::cast_from(0.35374814814814814815e-1_f64) * t22528 - F::cast_from(0.44218518518518518518e-1_f64) * t1153 * t22531 - F::cast_from(0.29479012345679012345e-1_f64) * t22534 - F::cast_from(0.35374814814814814815e-1_f64) * t22536 - F::cast_from(0.232171875e-2_f64) * t17627 * t22160 - F::new(0.9286875e-2) * t4202 * t22094;
    t22542
}
