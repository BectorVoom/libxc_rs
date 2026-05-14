//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 810/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk810<F: Float>(t13740: F, t484: F, t13847: F, t825: F, t826: F, t12161: F, t123: F, t883: F, t2684: F, t2685: F, t12213: F, t2464: F, t2465: F, t13851: F, t2013: F, t12240: F, t2679: F, t9800: F) -> (F, F, F, F, F, F, F) {
    let t47042 = t484 * t13740;
    let t47140 = t825 * t826 * t13847;
    let t47143 = t12161 * t123 * t883;
    let t47145 = t2684 * t2685 * t47143;
    let t47149 = t2684 * t2464 * t2465 * t12213;
    let t47151 = t2013 * t13851;
    let t47166 = t9800 * t12240 * t2679;
    (t47042, t47140, t47143, t47145, t47149, t47151, t47166)
}
