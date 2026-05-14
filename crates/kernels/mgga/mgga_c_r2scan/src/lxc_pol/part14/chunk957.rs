//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 957/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk957<F: Float>(t10950: F, t11015: F, t3434: F, t10648: F, t10649: F, t1375: F, t58: F, t597: F, t10650: F, t1654: F, t10673: F, t10674: F, t10676: F, t874: F, t10680: F, t10682: F) -> (F, F, F, F, F, F) {
    let t37483 = t3434 * t11015 * t10950;
    let t37488 = t10648 * t10649 * t58 * t1375 * t597;
    let t37495 = t10648 * t10649 * t10650 * t1654;
    let t37499 = t10673 * t10674 * t1375 * t10676;
    let t37501 = t1654 * t874;
    let t37503 = t10680 * t10682 * t37501;
    (t37483, t37488, t37495, t37499, t37501, t37503)
}
