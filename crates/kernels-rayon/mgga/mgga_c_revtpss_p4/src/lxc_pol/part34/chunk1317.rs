//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1317/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1317(t30138: f64, t7735: f64, t29576: f64, t7898: f64, t105866: f64, t114230: f64, t114238: f64, t114360: f64, t114404: f64, t114407: f64, t114410: f64, t114415: f64, t114417: f64, t114419: f64, t1519: f64, t1843: f64, t2007: f64, t22639: f64, t25045: f64, t28030: f64, t29569: f64, t30116: f64, t30119: f64, t33602: f64, t4248: f64, t508: f64, t569: f64, t5887: f64, t5920: f64, t5921: f64, t651: f64, t6985: f64, t7732: f64, t7883: f64) -> f64 {
    let t114421 = 12.0_f64 * t30138 * t7735;
    let t114427 = 6.0_f64 * t7898 * t29576;
    let t114431 = -6.0_f64 * t105866 * t1519 - 12.0_f64 * t28030 * t5887 - 6.0_f64 * t6985 * t25045 - t114230 - 6.0_f64 * t651 * t7883 * t5920 - 6.0_f64 * t33602 * t5921 - t114238 + t114404 * t569 - t114407 - t114410 - 12.0_f64 * t4248 * t30116 - t114415 - t114417 - t114419 - t114421 - 6.0_f64 * t7732 * t30119 - 6.0_f64 * t22639 * t2007 + t114427 - t114360 * t508 - 3.0_f64 * t29569 * t1843;
    t114431
}
