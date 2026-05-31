//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 472/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk472<F: Float>(t2969: F, t2846: F, t320: F, t963: F, t315: F, t2904: F) -> (F, F, F, F, F, F, F, F) {
    let t2970 = F::cast_from(1.0_f64) / t2969;
    let t2974 = F::cast_from(0.12361111111111111111e-1_f64) * t2846;
    let t2985 = t963 * t320;
    let t2986 = F::cast_from(1.0_f64) / t2985;
    let t2987 = t315 * t2986;
    let t2994 = F::cast_from(0.40256666666666666667e0_f64) * t2846;
    let t3001 = F::cast_from(0.137975e0_f64) * t2904;
    let t3010 = t963 * t963;
    let t3011 = F::cast_from(1.0_f64) / t3010;
    (t2970, t2974, t2986, t2987, t2994, t3001, t3010, t3011)
}
