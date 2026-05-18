//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1443/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1443<F: Float>(t2869: F, t36288: F, t36290: F, t36295: F, t36303: F, t36304: F, t36312: F, t36314: F, t36318: F, t36326: F, t36455: F, t36462: F, t38699: F, t38702: F, t38705: F, t38706: F, t38708: F, t38710: F, t38834: F, t38838: F, t3916: F) -> F {
    let t38893 = t2869 * t3916 + t36288 + t36290 + t36295 + t36303 - t36304 + t36312 + t36314 + t36318 + t36326 - t36455 + t36462 - t38699 + t38702 - t38705 - t38706 - t38708 + t38710 - t38834 + t38838;
    t38893
}
