//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1037/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1037<F: Float>(t1536: F, t8369: F, t8366: F, t4463: F, t8365: F, t6540: F, t6544: F, t14800: F, t8349: F, t1210: F, t8375: F, t2297: F, t5788: F, t14757: F, t14798: F, t21748: F, t21872: F, t25685: F, t25687: F, t25760: F, t25762: F, t4436: F, t4461: F, t4471: F, t4478: F, t6554: F, t6561: F) -> (F,) {
    let t27578 = t8369 * t1536;
    let t27581 = t8366 * t1536;
    let t27584 = t8365 * t4463;
    let t27585 = t27584 * t1536;
    let t27588 = t6544 * t6540;
    let t27591 = t8349 * t14800;
    let t27592 = t27591 * t1536;
    let t27599 = t8375 * t1210;
    let t27602 = t2297 * t5788;
    let t27605 = t25685 - t25687 - t25760 - t25762 - 0.19298809906722418785e3 * t14757 * t27578 - 2.0 * t4436 * t27581 + 0.32164683177870697974e2 * t4461 * t27585 + 0.64329366355741395948e2 * t4461 * t27588 + 0.20691336878655965246e4 * t14798 * t27592 - 0.23392893589820816284e1 * t21872 * t6554 + 0.346315117987517266e2 * t21748 * t6561 + 0.35089340384731224426e1 * t4478 * t27599 - 0.23392893589820816284e1 * t4471 * t27602;
    (t27605,)
}
