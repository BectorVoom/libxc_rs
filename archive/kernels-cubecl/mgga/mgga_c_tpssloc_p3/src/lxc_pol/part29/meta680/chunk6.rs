//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2292/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2292<F: Float>(t1089: F, t1751: F, t7327: F, t1653: F, t7330: F, t85822: F, t3961: F, t131: F, t1419: F, t23598: F, t467: F, t14165: F, t15702: F, t15776: F, t1755: F, t24589: F, t24667: F, t24785: F, t24817: F, t24823: F, t24849: F, t24852: F, t27507: F, t27531: F, t27550: F, t27551: F, t27643: F, t3248: F, t3252: F, t7373: F, t7375: F, t7376: F, t8066: F, t85820: F, t86015: F, t86037: F, t86059: F) -> (F, F) {
    let t94837 = t7327 * t1751 * t1089;
    let t94847 = t85822 * t1653 * t7330;
    let t94850 = t3961 * t7330;
    let t94858 = t1419 * t23598 * t131 * t467;
    let t94867 = -F::cast_from(0.16449340668482264365e-1_f64) * t24589 * t27550 * t27551 * t14165 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t86059 * t8066 - F::cast_from(0.27415567780803773942e-2_f64) * t24849 * t27531 * t7376 * t3252 - F::cast_from(0.54831135561607547884e-2_f64) * t24849 * t27531 * t7376 * t3248 - F::cast_from(0.54831135561607547884e-2_f64) * t24849 * t94837 * t24852 + F::cast_from(0.54831135561607547884e-2_f64) * t86037 * t24667 * t1755 * t27643 * t15702 + F::cast_from(0.54831135561607547884e-2_f64) * t85820 * t94847 - F::cast_from(0.10966227112321509577e-1_f64) * t24849 * t86015 * t94850 - F::cast_from(0.43864908449286038306e-1_f64) * t27507 * t24785 - F::cast_from(0.43864908449286038306e-1_f64) * t94858 * t24817 + F::cast_from(0.21932454224643019153e-1_f64) * t94858 * t24823 + F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t7375 * t15776 * t7376;
    (t94850, t94867)
}
