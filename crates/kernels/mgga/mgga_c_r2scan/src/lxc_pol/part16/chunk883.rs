//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 883/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk883<F: Float>(t11629: F, t3275: F, t3277: F, t10918: F, t2867: F, t11479: F, t3262: F, t3264: F, t3332: F, t7629: F, t7628: F, t8156: F, t6165: F, t8160: F, t7615: F, t7614: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11631 = t3275 * t11629 * t3277;
    let t11632 = 5.0 / 16.0 * t11631;
    let t11634 = t3275 * t10918 * t2867;
    let t11635 = t11634 / 4.0;
    let t11637 = t3262 * t11479 * t3264;
    let t11638 = 3.0 / 4.0 * t11637;
    let t11640 = t3332 * t7629;
    let t11641 = t7628 * t11640;
    let t11643 = t3332 * t8156;
    let t11644 = t6165 * t11643;
    let t11646 = t3332 * t8160;
    let t11647 = t6165 * t11646;
    let t11649 = t3332 * t7615;
    let t11650 = t7614 * t11649;
    (t11631, t11632, t11634, t11635, t11637, t11638, t11640, t11641, t11643, t11644, t11646, t11647, t11649, t11650)
}
