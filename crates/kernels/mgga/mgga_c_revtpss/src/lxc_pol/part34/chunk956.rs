//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 956/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk956<F: Float>(t22984: F, t23058: F, t1343: F, t1450: F, t198: F, t22768: F, t22791: F, t22809: F, t22919: F, t22920: F, t22921: F, t22922: F, t532: F, t9394: F, t9396: F, t9409: F, t9412: F, t9415: F, t9421: F, t9427: F) -> (F, F) {
    let t23059 = t22984 + t23058;
    let t23063 = t1450 * t198 * t23059 * t532 + F::new(3.0) * t1343 * t198 * t22809 - t22768 + t22791 + t22919 - t22920 + t22921 + t22922 + t9394 - t9396 + t9409 - t9412 - t9415 + t9421 - t9427;
    (t23059, t23063)
}
