//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 896/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk896<F: Float>(t1445: F, t2087: F, t37200: F, t935: F, t11016: F, t3651: F, t15498: F, t15499: F, t44707: F, t590: F, t2679: F, t3626: F, t9800: F) -> (F, F, F, F) {
    let t45264 = F::cast_from(0.69017266717057349418e1_f64) * t2087 * t1445 * t37200 * t935;
    let t45269 = F::cast_from(0.16683561977530199113e1_f64) * t3651 * t11016;
    let t45277 = F::cast_from(0.61348681526273199482e1_f64) * t15498 * t15499 * t44707 * t590;
    let t45285 = t9800 * t3626 * t2679;
    (t45264, t45269, t45277, t45285)
}
