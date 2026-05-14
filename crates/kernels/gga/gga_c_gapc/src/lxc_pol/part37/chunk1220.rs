//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1220/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1220<F: Float>(t2469: F, t2822: F, t36297: F, t36303: F, t36304: F, t36307: F, t36309: F, t36312: F, t36314: F, t36316: F, t36318: F, t36320: F, t36323: F, t36324: F, t38699: F, t38702: F, t38705: F, t38706: F, t3914: F, t7053: F) -> (F,) {
    let t38831 = 2.0 * t2469 * t2822 * t3914 - t3914 * t7053 + t36297 - t36303 + t36304 + t36307 + t36309 - t36312 - t36314 - t36316 - t36318 + t36320 + t36323 - t36324 + t38699 - t38702 + t38705 + t38706;
    (t38831,)
}
