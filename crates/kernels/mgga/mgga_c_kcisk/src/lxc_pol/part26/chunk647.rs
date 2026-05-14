//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 647/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk647<F: Float>(t1537: F, t6540: F, t2292: F, t4463: F, t1536: F, t1203: F, t2097: F, t1210: F, t2297: F, t1212: F, t5788: F, t2105: F, t3725: F, t1529: F, t1538: F, t1542: F, t1543: F, t2293: F, t4431: F, t4436: F, t4461: F, t4468: F, t4471: F, t4478: F, t516: F, t5714: F, t5717: F, t5719: F, t5722: F, t5751: F, t5755: F, t5762: F, t6515: F, t6518: F, t6523: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6541 = t6540 * t1537;
    let t6544 = t2292 * t4463;
    let t6545 = t6544 * t1536;
    let t6549 = t2097 * t1203;
    let t6554 = t2297 * t1210;
    let t6557 = t5788 * t1212;
    let t6560 = t2105 * t3725;
    let t6561 = t6560 * t1210;
    let t6564 = -0.3109e-1 * t6515 * t516 + 1.0 * t6518 * t1538 + 1.0 * t4431 * t2293 - 2.0 * t4436 * t6523 + 1.0 * t1529 * t6541 + 0.32164683177870697974e2 * t4461 * t6545 + t5714 - t5717 - t5719 + t5722 - t5751 - t5755 - 0.19751789702565206229e-1 * t5762 + 0.58482233974552040708e0 * t6549 * t1543 + 0.58482233974552040708e0 * t4468 * t2297 - 0.11696446794910408142e1 * t4471 * t6554 + 0.58482233974552040708e0 * t1542 * t6557 + 0.17315755899375863299e2 * t4478 * t6561;
    (t6541, t6544, t6545, t6549, t6554, t6557, t6560, t6561, t6564)
}
