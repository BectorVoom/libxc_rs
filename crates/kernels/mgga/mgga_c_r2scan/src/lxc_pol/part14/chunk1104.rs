//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1104/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1104<F: Float>(t3261: F, t5086: F, t97: F, t481: F, t792: F, t983: F, t10609: F, t1561: F, t2625: F, t11531: F, t11584: F, t37365: F) -> (F, F, F, F, F, F) {
    let t39190 = t97 * t3261 * t5086;
    let t39192 = t983 * t481 * t792;
    let t39197 = t97 * t10609 * t1561;
    let t39198 = t2625 * t792;
    let t39209 = t11531 * t792;
    let t39215 = t37365 * t11584;
    (t39190, t39192, t39197, t39198, t39209, t39215)
}
