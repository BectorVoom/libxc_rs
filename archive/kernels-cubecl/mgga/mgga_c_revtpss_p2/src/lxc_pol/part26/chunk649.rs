//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 649/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk649<F: Float>(t3088: F, t4890: F, t3299: F, t1043: F, t3154: F, t3317: F, t357: F, t999: F, t1012: F, t1014: F, t3252: F, t3298: F, t378: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4891 = t3088 * t4890;
    let t4892 = t3299 * t4891;
    let t4894 = t3154 * t1043;
    let t4899 = t3317 * t4891;
    let t4900 = t1043 * t357;
    let t4910 = t357 * t999;
    let t4915 = t1012 * t1014;
    let t4919 = t1012 * t3252;
    let t4980 = t3298 * t378;
    (t4891, t4892, t4894, t4899, t4900, t4910, t4915, t4919, t4980)
}
