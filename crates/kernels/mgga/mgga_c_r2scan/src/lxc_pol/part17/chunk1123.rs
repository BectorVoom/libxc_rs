//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1123/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1123<F: Float>(t38299: F, t897: F, t10680: F, t38301: F, t11587: F, t37501: F, t10673: F, t11591: F, t37505: F, t10935: F, t2810: F, t3446: F) -> (F, F, F, F) {
    let t40409 = t38299 * t897;
    let t40411 = t10680 * t40409 * t38301;
    let t40425 = t10680 * t11587 * t37501;
    let t40428 = t10673 * t11591 * t37505;
    let t40434 = t3446 * t10935 * t2810;
    (t40411, t40425, t40428, t40434)
}
