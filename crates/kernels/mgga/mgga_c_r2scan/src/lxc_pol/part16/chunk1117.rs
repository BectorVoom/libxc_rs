//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1117/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1117<F: Float>(t40345: F, t104: F, t920: F, t38299: F, t897: F, t10680: F, t38301: F, t3618: F, t481: F, t3270: F, t11587: F, t37501: F) -> (F, F, F, F, F) {
    let t40346 = F::new(0.10248087766267884742e-3) * t40345;
    let t40393 = t104 * t920;
    let t40409 = t38299 * t897;
    let t40411 = t10680 * t40409 * t38301;
    let t40420 = t3618 * t481;
    let t40421 = t3270 * t40420;
    let t40425 = t10680 * t11587 * t37501;
    (t40346, t40393, t40411, t40421, t40425)
}
