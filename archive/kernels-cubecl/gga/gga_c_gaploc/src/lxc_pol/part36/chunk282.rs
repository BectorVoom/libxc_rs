//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 282/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk282<F: Float>(t2304: F, t494: F, t484: F, t885: F, t417: F, t78: F, t119: F, t481: F) -> (F, F, F, F) {
    let t2305 = t2304 * t494;
    let t2308 = t484 * t885;
    let t2310 = t78 * t417;
    let t2312 = t481 * t2310 * t119;
    (t2305, t2308, t2310, t2312)
}
