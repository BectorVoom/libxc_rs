//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1054/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1054<F: Float>(t10673: F, t11591: F, t40317: F, t3446: F, t3453: F, t9063: F, t9066: F, t9069: F, t9072: F, t10648: F, t10649: F, t11582: F, t2768: F, t3033: F, t58: F, t597: F) -> (F, F, F, F, F, F, F) {
    let t43878 = t10673 * t11591 * t40317;
    let t43887 = t3446 * t3453 * t9063;
    let t43892 = t3446 * t3453 * t9066;
    let t43895 = t3446 * t3453 * t9069;
    let t43898 = t3446 * t3453 * t9072;
    let t43902 = t10648 * t10649 * t11582 * t2768;
    let t43907 = t10648 * t10649 * t58 * t3033 * t597;
    (t43878, t43887, t43892, t43895, t43898, t43902, t43907)
}
