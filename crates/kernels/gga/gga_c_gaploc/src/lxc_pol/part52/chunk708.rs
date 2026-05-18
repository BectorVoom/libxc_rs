//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 708/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk708<F: Float>(t123: F, t3720: F, t883: F, t2685: F, t2684: F, t969: F, t825: F, t2610: F, t2365: F, t2033: F, t12252: F, t959: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13846 = t3720 * t123;
    let t13847 = t13846 * t883;
    let t13848 = t2685 * t13847;
    let t13849 = t2684 * t13848;
    let t13851 = t969 * t13847;
    let t13852 = t825 * t13851;
    let t13891 = t2610 * t3720;
    let t13892 = t2365 * t13891;
    let t13893 = t2033 * t13892;
    let t13895 = t12252 * t959;
    (t13847, t13848, t13849, t13851, t13852, t13891, t13892, t13893, t13895)
}
