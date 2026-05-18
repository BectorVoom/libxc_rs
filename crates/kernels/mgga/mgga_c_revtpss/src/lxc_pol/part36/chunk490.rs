//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 490/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk490<F: Float>(t3153: F, t3302: F, t3154: F, t1035: F, t3140: F, t342: F, t357: F, t389: F, t1941: F, t268: F, t404: F, t1263: F, t159: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3303 = t3153 * t3302;
    let t3304 = t3303 * t3154;
    let t3316 = t3140 * t1035;
    let t3317 = t342 * t3316;
    let t3318 = t3303 * t357;
    let t3335 = t389 * t389;
    let t3336 = F::new(1.0) / t3335;
    let t3356 = t268 * t1941 * t404;
    let t3357 = F::new(0.23744444444444444444e-1) * t3356;
    let t3360 = t159 * t1263;
    (t3303, t3304, t3316, t3317, t3318, t3335, t3336, t3356, t3357, t3360)
}
