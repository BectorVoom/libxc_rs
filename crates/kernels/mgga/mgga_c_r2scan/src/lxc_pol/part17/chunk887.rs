//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 887/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk887<F: Float>(t2207: F, t3328: F, t3613: F, t2201: F, t3336: F, t3602: F, t1060: F, t2719: F, t1058: F, t2583: F, t3308: F, t574: F, t2559: F, t3295: F, t2563: F, t10776: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11788 = t2207 * t3613 * t3328;
    let t11791 = t2201 * t3336 * t3602;
    let t11793 = t1060 * t2719;
    let t11795 = t2201 * t1058 * t11793;
    let t11797 = t3308 * t2583;
    let t11798 = t574 * t11797;
    let t11800 = t3295 * t2559;
    let t11802 = t3308 * t2563;
    let t11803 = t10776 * t11802;
    (t11788, t11791, t11793, t11795, t11797, t11798, t11800, t11802, t11803)
}
