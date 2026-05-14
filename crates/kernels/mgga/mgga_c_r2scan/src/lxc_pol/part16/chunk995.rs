//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 995/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk995<F: Float>(t10954: F, t11564: F, t3446: F, t11015: F, t11568: F, t3434: F, t10680: F, t10681: F, t10683: F, t2482: F, t10673: F, t10674: F, t10676: F, t104: F, t920: F, t38299: F, t897: F) -> (F, F, F, F, F, F) {
    let t40331 = t3446 * t10954 * t11564;
    let t40334 = t3434 * t11015 * t11568;
    let t40341 = t10680 * t10681 * t2482 * t10683;
    let t40342 = 0.72042316457491791906e-3 * t40341;
    let t40345 = t10673 * t10674 * t2482 * t10676;
    let t40346 = 0.10248087766267884742e-3 * t40345;
    let t40393 = t104 * t920;
    let t40409 = t38299 * t897;
    (t40331, t40334, t40342, t40346, t40393, t40409)
}
