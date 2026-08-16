//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 806/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk806<F: Float>(t32215: F, t5539: F, t9647: F, t2563: F, t3487: F, t7284: F, t29277: F, t32607: F, t10639: F, t16879: F, t883: F, t10736: F, t7064: F) -> (F, F, F, F, F) {
    let t42956 = t9647 * t5539 * t32215;
    let t42960 = t9647 * t7284 * t3487 * t2563;
    let t42963 = t9647 * t29277 * t32607;
    let t42967 = t9647 * t16879 * t883 * t10639;
    let t42970 = t7064 * t29277 * t10736;
    (t42956, t42960, t42963, t42967, t42970)
}
