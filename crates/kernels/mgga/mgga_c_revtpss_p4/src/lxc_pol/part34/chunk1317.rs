//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1317/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1317<F: Float>(t30138: F, t7735: F, t29576: F, t7898: F, t105866: F, t114230: F, t114238: F, t114360: F, t114404: F, t114407: F, t114410: F, t114415: F, t114417: F, t114419: F, t1519: F, t1843: F, t2007: F, t22639: F, t25045: F, t28030: F, t29569: F, t30116: F, t30119: F, t33602: F, t4248: F, t508: F, t569: F, t5887: F, t5920: F, t5921: F, t651: F, t6985: F, t7732: F, t7883: F) -> F {
    let t114421 = F::cast_from(12.0_f64) * t30138 * t7735;
    let t114427 = F::cast_from(6.0_f64) * t7898 * t29576;
    let t114431 = -F::cast_from(6.0_f64) * t105866 * t1519 - F::cast_from(12.0_f64) * t28030 * t5887 - F::cast_from(6.0_f64) * t6985 * t25045 - t114230 - F::cast_from(6.0_f64) * t651 * t7883 * t5920 - F::cast_from(6.0_f64) * t33602 * t5921 - t114238 + t114404 * t569 - t114407 - t114410 - F::cast_from(12.0_f64) * t4248 * t30116 - t114415 - t114417 - t114419 - t114421 - F::cast_from(6.0_f64) * t7732 * t30119 - F::cast_from(6.0_f64) * t22639 * t2007 + t114427 - t114360 * t508 - F::cast_from(3.0_f64) * t29569 * t1843;
    t114431
}
