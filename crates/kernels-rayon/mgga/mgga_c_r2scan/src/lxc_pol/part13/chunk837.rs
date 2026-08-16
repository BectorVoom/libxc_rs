//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 837/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk837(t1234: f64, t910: f64, t551: f64, t552: f64, t1543: f64, t938: f64, t1600: f64, t2631: f64, t2185: f64, t1584: f64, t1588: f64, t1592: f64, t2122: f64, t2196: f64, t2646: f64, t2651: f64, t5136: f64, t574: f64, t576: f64, t6132: f64, t6178: f64, t6196: f64, t6215: f64, t6449: f64, t7535: f64, t7539: f64, t7544: f64, t7553: f64, t7557: f64, t7561: f64, t7566: f64) -> (f64, f64) {
    let t7569 = t910 * t1234;
    let t7571 = t551 * t552 * t7569;
    let t7576 = t938 * t1543;
    let t7578 = t551 * t552 * t7576;
    let t7582 = 0.12805040077930161442e0_f64 * t1600 * t2631;
    let t7583 = t938 * t2185;
    let t7585 = t551 * t552 * t7583;
    let t7588 = -0.10975748638225852664e0_f64 * t2122 * t7535 - 0.86682217400542685632e-1_f64 * t6132 * t7539 + 0.2600466522016280569e0_f64 * t1592 * t7544 + 0.64025200389650807209e-1_f64 * t6178 - 0.43341108700271342816e-1_f64 * t2651 * t1588 - t7553 - t7557 - 0.28914548798370980346e-3_f64 * t6196 - 0.12695991786046386926e-1_f64 * t6215 - 0.43341108700271342816e-1_f64 * t574 * t7561 - 0.86682217400542685632e-1_f64 * t7566 * t576 + 0.5200933044032561138e0_f64 * t2196 * t7571 - 0.86682217400542685632e-1_f64 * t1584 * t2646 - 0.5200933044032561138e0_f64 * t6449 * t7578 + t7582 - 0.2600466522016280569e0_f64 * t5136 * t7585;
    (t7569, t7588)
}
