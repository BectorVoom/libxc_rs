//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 875/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk875<F: Float>(t11563: F, t874: F, t3446: F, t3447: F, t122: F, t3434: F, t3437: F, t1103: F, t2461: F, t1053: F, t1102: F, t10935: F, t970: F, t58: F, t897: F, t597: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11564 = t11563 * t874;
    let t11566 = t3446 * t3447 * t11564;
    let t11568 = t11563 * t122;
    let t11570 = t3434 * t3437 * t11568;
    let t11572 = t1103 * t2461;
    let t11574 = t1102 * t1053 * t11572;
    let t11580 = t3446 * t10935 * t970;
    let t11582 = t58 * t897;
    let t11583 = t11582 * t597;
    (t11564, t11566, t11568, t11570, t11572, t11574, t11580, t11582, t11583)
}
