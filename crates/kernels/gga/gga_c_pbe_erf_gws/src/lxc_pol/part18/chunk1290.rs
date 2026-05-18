//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1290/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1290<F: Float>(t15322: F, t4414: F, t12198: F, t4002: F, t13796: F, t14637: F, t3737: F, t875: F, t11354: F, t11401: F, t1185: F, t12204: F, t12237: F, t13888: F, t14403: F, t14651: F, t14791: F, t15138: F, t2408: F, t27047: F, t27105: F, t3066: F, t3067: F, t3207: F, t35566: F, t53253: F, t53374: F, t53405: F, t53407: F, t53472: F, t56199: F, t8629: F, t8654: F, t8776: F, t9283: F, t938: F) -> F {
    let t56385 = t4414 * t15322;
    let t56400 = t12198 * t4002;
    let t56404 = t14637 * t13796 * t3737 * t875;
    let t56425 = -t53374 - F::new(7.0) / F::new(72.0) * t56385 + t53405 - t8629 * t27047 * t3067 * t56199 * t938 / F::new(48.0) - t8629 * t53472 / F::new(24.0) + t8776 * t1185 * t15138 / F::new(96.0) + t8654 * t27105 * t14403 / F::new(24.0) + F::new(7.0) / F::new(288.0) * t56400 + t53407 - F::new(5.0) / F::new(768.0) * t56404 - t2408 * t9283 * t13888 * t11401 / F::new(12.0) - t3066 * t9283 * t14791 * t11354 / F::new(16.0) - t2408 * t35566 * t14651 / F::new(12.0) + t3066 * t9283 * t53253 * t12204 / F::new(4.0) + t3207 * t9283 * t13888 * t12237 / F::new(8.0);
    t56425
}
