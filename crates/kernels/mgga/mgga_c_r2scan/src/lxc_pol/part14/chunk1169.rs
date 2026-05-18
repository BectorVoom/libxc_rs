//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1169/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1169<F: Float>(t40393: F, t97: F, t1234: F, t3582: F, t38299: F, t897: F, t10680: F, t38301: F, t11531: F, t481: F, t11587: F, t37501: F) -> (F, F, F, F, F) {
    let t40394 = t97 * t40393;
    let t40397 = t3582 * t1234;
    let t40409 = t38299 * t897;
    let t40411 = t10680 * t40409 * t38301;
    let t40416 = t11531 * t481;
    let t40425 = t10680 * t11587 * t37501;
    (t40394, t40397, t40411, t40416, t40425)
}
