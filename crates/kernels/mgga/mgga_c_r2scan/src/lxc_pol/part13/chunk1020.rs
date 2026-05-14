//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1020/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1020<F: Float>(t1577: F, t3308: F, t8034: F, t3295: F, t7524: F, t10760: F, t25670: F, t6093: F, t37718: F, t37721: F, t39628: F, t39630: F, t39632: F, t39635: F, t39637: F, t39640: F, t39642: F) -> (F,) {
    let t39645 = t1577 * t3308 * t8034;
    let t39647 = t3295 * t7524;
    let t39650 = t6093 * t10760 * t25670;
    let t39652 = -0.47609969197673950972e-2 * t37718 - 0.14282990759302185292e-1 * t37721 + t39628 + t39630 + 0.26198215989259945075e-1 * t39632 - 0.12713391885412927226e1 * t39635 - 0.16463622957338778997e-1 * t39637 - 0.32927245914677557994e-1 * t39640 + 0.58544643236296698113e-1 * t39642 + 0.26004665220162805689e0 * t39645 + 0.16463622957338778996e0 * t39647 - 0.65495539973149862688e-2 * t39650;
    (t39652,)
}
