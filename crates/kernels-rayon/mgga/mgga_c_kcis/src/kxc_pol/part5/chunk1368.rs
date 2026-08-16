//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1368/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1368(t3393: f64, t7373: f64, t143: f64, t21453: f64, t7369: f64, t12406: f64, t4219: f64, t6281: f64, t7361: f64, t7365: f64, t1153: f64, t12397: f64, t12401: f64, t17627: f64, t22054: f64, t22059: f64, t22094: f64, t22134: f64, t22142: f64, t22160: f64, t22165: f64, t22169: f64, t4202: f64, t545: f64, t5947: f64, t5958: f64) -> f64 {
    let t22514 = t3393 * t7373;
    let t22520 = t21453 * t143;
    let t22528 = t3393 * t7369;
    let t22531 = t4219 * t12406 * t6281;
    let t22534 = t3393 * t7361;
    let t22536 = t3393 * t7365;
    let t22542 = -0.46434375e-2_f64 * t5947 * t22165 + 0.9286875e-2_f64 * t5947 * t22169 - 0.17687407407407407407e-1_f64 * t22514 - 0.9286875e-2_f64 * t4202 * t22059 + 0.1857375e-1_f64 * t4202 * t22134 + 0.619125e-2_f64 * t22520 * t545 + 0.46434375e-2_f64 * t5947 * t22054 + 0.24765e-1_f64 * t5958 * t22142 + 0.88437037037037037037e-2_f64 * t12397 - t12401 + 0.35374814814814814815e-1_f64 * t22528 - 0.44218518518518518518e-1_f64 * t1153 * t22531 - 0.29479012345679012345e-1_f64 * t22534 - 0.35374814814814814815e-1_f64 * t22536 - 0.232171875e-2_f64 * t17627 * t22160 - 0.9286875e-2_f64 * t4202 * t22094;
    t22542
}
