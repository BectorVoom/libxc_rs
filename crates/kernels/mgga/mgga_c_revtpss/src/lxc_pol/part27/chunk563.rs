//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 563/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk563<F: Float>(t225: F, t3259: F, t385: F, t1071: F, t342: F, t1077: F, t384: F, t1096: F, t1086: F, t989: F, t1082: F, t3059: F) -> (F, F, F, F, F, F, F) {
    let t3261 = t3259 * t225 * t385;
    let t3264 = t342 * t1071;
    let t3268 = F::new(1.0) / t1077 / t384;
    let t3269 = t225 * t3268;
    let t3270 = t1096 * t1096;
    let t3271 = t3269 * t3270;
    let t3278 = t989 * t1086;
    let t3283 = t1082 * t3059;
    (t3261, t3264, t3269, t3270, t3271, t3278, t3283)
}
