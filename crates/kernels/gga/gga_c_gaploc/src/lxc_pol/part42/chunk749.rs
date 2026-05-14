//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 749/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk749<F: Float>(t1445: F, t2087: F, t37200: F, t935: F, t11016: F, t3651: F, t15498: F, t15499: F, t44707: F, t590: F, t2679: F, t3626: F, t9800: F, t43446: F, t43454: F, t2639: F, t3614: F, t7284: F, t787: F) -> (F, F, F, F, F, F, F) {
    let t45264 = 0.69017266717057349418e1 * t2087 * t1445 * t37200 * t935;
    let t45269 = 0.16683561977530199113e1 * t3651 * t11016;
    let t45277 = 0.61348681526273199482e1 * t15498 * t15499 * t44707 * t590;
    let t45285 = t9800 * t3626 * t2679;
    let t45287 = 0.41708904943825497782e0 * t43446;
    let t45288 = 0.35750489951850426669e0 * t43454;
    let t45298 = 0.25025342966295298669e1 * t787 * t7284 * t3614 * t2639;
    (t45264, t45269, t45277, t45285, t45287, t45288, t45298)
}
