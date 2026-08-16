//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 824/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk824<F: Float>(t2365: F, t32215: F, t6111: F, t13149: F, t2464: F, t825: F, t10893: F, t2628: F, t13150: F, t2013: F, t10007: F, t2925: F, t9438: F) -> (F, F, F, F, F) {
    let t44012 = t6111 * t2365 * t32215;
    let t44045 = t825 * t2464 * t13149;
    let t44070 = t10893 * t2628;
    let t44084 = t2013 * t13150;
    let t44088 = t825 * t9438 * t10007 * t2925;
    (t44012, t44045, t44070, t44084, t44088)
}
