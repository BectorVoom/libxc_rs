//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1427/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1427<F: Float>(t12295: F, t987: F, t36288: F, t36290: F, t36295: F, t36299: F, t36303: F, t36304: F, t36305: F, t36312: F, t36314: F, t36318: F, t36326: F, t36453: F, t36455: F, t36462: F, t36465: F, t36479: F, t36481: F, t36893: F, t37302: F) -> F {
    let t38853 = t987 * t12295;
    let t38854 = t36288 + t36290 + t36295 + t36299 + t36303 - t36304 - t36305 + t36312 + t36314 + t36318 + t36326 + t36453 - t36455 + t36462 + t36465 + t38853 + t36479 - t36481 - t36893 - t37302;
    t38854
}
