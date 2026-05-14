//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 774/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk774<F: Float>(t2832: F, t537: F, t255: F, t571: F, t1234: F, t910: F, t551: F, t552: F, t1543: F, t938: F, t1600: F, t2631: F, t2185: F, t1584: F, t1588: F, t1592: F, t2122: F, t2196: F, t2646: F, t2651: F, t5136: F, t574: F, t576: F, t6132: F, t6178: F, t6196: F, t6215: F, t6449: F, t7535: F, t7539: F, t7544: F, t7553: F, t7557: F, t7561: F) -> (F, F, F) {
    let t7564 = t537 * t2832;
    let t7566 = t571 * t7564 * t255;
    let t7569 = t910 * t1234;
    let t7571 = t551 * t552 * t7569;
    let t7576 = t938 * t1543;
    let t7578 = t551 * t552 * t7576;
    let t7582 = 0.12805040077930161442e0 * t1600 * t2631;
    let t7583 = t938 * t2185;
    let t7585 = t551 * t552 * t7583;
    let t7588 = -0.10975748638225852664e0 * t2122 * t7535 - 0.86682217400542685632e-1 * t6132 * t7539 + 0.2600466522016280569e0 * t1592 * t7544 + 0.64025200389650807209e-1 * t6178 - 0.43341108700271342816e-1 * t2651 * t1588 - t7553 - t7557 - 0.28914548798370980346e-3 * t6196 - 0.12695991786046386926e-1 * t6215 - 0.43341108700271342816e-1 * t574 * t7561 - 0.86682217400542685632e-1 * t7566 * t576 + 0.5200933044032561138e0 * t2196 * t7571 - 0.86682217400542685632e-1 * t1584 * t2646 - 0.5200933044032561138e0 * t6449 * t7578 + t7582 - 0.2600466522016280569e0 * t5136 * t7585;
    (t7566, t7569, t7588)
}
