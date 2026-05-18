//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 983/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk983<F: Float>(t11563: F, t122: F, t3434: F, t3437: F, t1103: F, t2461: F, t1053: F, t1102: F, t10935: F, t3446: F, t970: F, t58: F, t897: F) -> (F, F, F, F, F, F) {
    let t11568 = t11563 * t122;
    let t11570 = t3434 * t3437 * t11568;
    let t11572 = t1103 * t2461;
    let t11574 = t1102 * t1053 * t11572;
    let t11580 = t3446 * t10935 * t970;
    let t11582 = t58 * t897;
    (t11568, t11570, t11572, t11574, t11580, t11582)
}
