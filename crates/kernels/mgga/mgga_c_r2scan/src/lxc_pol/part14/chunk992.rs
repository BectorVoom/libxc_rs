//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 992/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk992<F: Float>(t38341: F, t38346: F, t38349: F, t38362: F, t11554: F, t2262: F, t6897: F, t910: F, t2330: F, t3261: F, t5086: F, t97: F, t481: F, t792: F, t983: F, t10609: F, t1561: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39129 = 0.2439011983326002265e-2 * t38341;
    let t39130 = 0.18292589874945016987e-2 * t38346;
    let t39131 = 0.30487649791575028312e-3 * t38349;
    let t39134 = 0.91462949374725084936e-3 * t38362;
    let t39178 = t11554 * t2262;
    let t39182 = t6897 * t910;
    let t39183 = t39182 * t2330;
    let t39190 = t97 * t3261 * t5086;
    let t39192 = t983 * t481 * t792;
    let t39197 = t97 * t10609 * t1561;
    (t39129, t39130, t39131, t39134, t39178, t39183, t39190, t39192, t39197)
}
