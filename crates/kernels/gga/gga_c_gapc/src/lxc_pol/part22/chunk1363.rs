//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1363/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1363<F: Float>(t2469: F, t2822: F, t36303: F, t36304: F, t36305: F, t36307: F, t36309: F, t36312: F, t36314: F, t36316: F, t36318: F, t36320: F, t36323: F, t36324: F, t36326: F, t36331: F, t36453: F, t36455: F, t36457: F, t3846: F) -> F {
    let t36458 = F::new(2.0) * t2469 * t2822 * t3846 - t36303 + t36304 + t36305 + t36307 + t36309 - t36312 - t36314 - t36316 - t36318 + t36320 + t36323 - t36324 - t36326 + t36331 - t36453 + t36455 + t36457;
    t36458
}
