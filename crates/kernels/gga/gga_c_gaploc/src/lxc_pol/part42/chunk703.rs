//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 703/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk703<F: Float>(t2558: F, t33348: F, t9647: F, t13182: F, t1841: F, t2563: F, t3487: F, t7284: F, t29277: F, t32607: F, t10639: F, t16879: F, t883: F, t10736: F, t7064: F, t10635: F, t2554: F) -> (F, F, F, F, F, F, F) {
    let t42942 = t9647 * t33348 * t2558;
    let t42953 = t1841 * t13182;
    let t42960 = t9647 * t7284 * t3487 * t2563;
    let t42963 = t9647 * t29277 * t32607;
    let t42967 = t9647 * t16879 * t883 * t10639;
    let t42970 = t7064 * t29277 * t10736;
    let t42973 = t7064 * t10635 * t2554;
    (t42942, t42953, t42960, t42963, t42967, t42970, t42973)
}
