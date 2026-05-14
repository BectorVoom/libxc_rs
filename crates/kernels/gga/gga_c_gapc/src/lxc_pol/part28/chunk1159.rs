//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1159/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1159<F: Float>(t209: F, t36345: F, t36359: F, t36374: F, t36389: F, t36420: F, t36434: F, t36449: F, t3537: F, t8598: F, t12291: F, t7056: F, t2469: F, t2822: F, t36303: F, t36304: F, t36305: F, t36307: F, t36309: F, t36312: F, t36314: F, t36316: F, t36318: F, t36320: F, t36323: F, t36324: F, t36326: F, t36331: F, t3846: F) -> (F, F, F) {
    let t36453 = (t36345 + t36359 + t36374 + t36389 + t36420 + t36434 + t36449) * t209;
    let t36455 = 2.0 * t8598 * t3537;
    let t36457 = 4.0 * t7056 * t12291;
    let t36458 = 2.0 * t2469 * t2822 * t3846 - t36303 + t36304 + t36305 + t36307 + t36309 - t36312 - t36314 - t36316 - t36318 + t36320 + t36323 - t36324 - t36326 + t36331 - t36453 + t36455 + t36457;
    (t36453, t36455, t36458)
}
