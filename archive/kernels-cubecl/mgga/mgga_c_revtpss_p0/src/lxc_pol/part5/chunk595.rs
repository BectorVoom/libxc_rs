//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 595/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk595<F: Float>(t2852: F, t3252: F, t1071: F, t342: F, t1077: F, t384: F, t225: F) -> (F, F, F, F) {
    let t3253 = t3252 * t2852;
    let t3264 = t342 * t1071;
    let t3268 = F::cast_from(1.0_f64) / t1077 / t384;
    let t3269 = t225 * t3268;
    (t3253, t3264, t3268, t3269)
}
