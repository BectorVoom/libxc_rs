//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 983/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk983<F: Float>(t1060: F, t11780: F, t783: F, t2201: F, t3324: F, t3613: F, t2207: F, t3328: F, t3336: F, t3602: F, t2719: F, t1058: F) -> (F, F, F, F, F, F) {
    let t11782 = t783 * t11780 * t1060;
    let t11785 = t2201 * t3613 * t3324;
    let t11788 = t2207 * t3613 * t3328;
    let t11791 = t2201 * t3336 * t3602;
    let t11793 = t1060 * t2719;
    let t11795 = t2201 * t1058 * t11793;
    (t11782, t11785, t11788, t11791, t11793, t11795)
}
