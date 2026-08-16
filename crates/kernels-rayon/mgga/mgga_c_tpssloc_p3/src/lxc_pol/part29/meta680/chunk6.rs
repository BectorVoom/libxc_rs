//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2292/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2292(t1089: f64, t1751: f64, t7327: f64, t1653: f64, t7330: f64, t85822: f64, t3961: f64, t131: f64, t1419: f64, t23598: f64, t467: f64, t14165: f64, t15702: f64, t15776: f64, t1755: f64, t24589: f64, t24667: f64, t24785: f64, t24817: f64, t24823: f64, t24849: f64, t24852: f64, t27507: f64, t27531: f64, t27550: f64, t27551: f64, t27643: f64, t3248: f64, t3252: f64, t7373: f64, t7375: f64, t7376: f64, t8066: f64, t85820: f64, t86015: f64, t86037: f64, t86059: f64) -> (f64, f64) {
    let t94837 = t7327 * t1751 * t1089;
    let t94847 = t85822 * t1653 * t7330;
    let t94850 = t3961 * t7330;
    let t94858 = t1419 * t23598 * t131 * t467;
    let t94867 = -0.16449340668482264365e-1_f64 * t24589 * t27550 * t27551 * t14165 + 0.27415567780803773942e-2_f64 * t24589 * t86059 * t8066 - 0.27415567780803773942e-2_f64 * t24849 * t27531 * t7376 * t3252 - 0.54831135561607547884e-2_f64 * t24849 * t27531 * t7376 * t3248 - 0.54831135561607547884e-2_f64 * t24849 * t94837 * t24852 + 0.54831135561607547884e-2_f64 * t86037 * t24667 * t1755 * t27643 * t15702 + 0.54831135561607547884e-2_f64 * t85820 * t94847 - 0.10966227112321509577e-1_f64 * t24849 * t86015 * t94850 - 0.43864908449286038306e-1_f64 * t27507 * t24785 - 0.43864908449286038306e-1_f64 * t94858 * t24817 + 0.21932454224643019153e-1_f64 * t94858 * t24823 + 0.16449340668482264365e-1_f64 * t7373 * t7375 * t15776 * t7376;
    (t94850, t94867)
}
