//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 515/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk515<F: Float>(t2465: F, t2571: F, t2464: F, t825: F, t1645: F, t9740: F, t2194: F, t3308: F, t7068: F, t883: F, t1967: F, t7810: F) -> (F, F, F, F, F) {
    let t9850 = t2465 * t2571;
    let t9851 = t2464 * t9850;
    let t9852 = t825 * t9851;
    let t9858 = t1645 * t9740;
    let t9873 = t2194 * t3308;
    let t9889 = t883 * t7068;
    let t9890 = t1967 * t9889;
    let t9891 = t7810 * t9890;
    (t9852, t9858, t9873, t9889, t9891)
}
