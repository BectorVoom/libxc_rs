//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1065/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1065<F: Float>(t11957: F, t3138: F, t3166: F, t3219: F, t3235: F, t3703: F, t6: F, t6366: F, t875: F, t11514: F, t11944: F, t11947: F, t11949: F, t11953: F, t2277: F, t2312: F, t2343: F, t6592: F, t6597: F, t9592: F, t9598: F) -> (F, F, F, F, F, F) {
    let t11959 = t3138 * t11957 / F::new(24.0);
    let t11961 = t3235 * t3219 * t3166;
    let t11964 = t6 * t3703;
    let t11966 = t6366 * t11964 * t875;
    let t11970 = t3235 * t11514 * t875;
    let t11973 = -F::new(35.0) / F::new(1152.0) * t11944 + t11947 - t6592 - t6597 - t2277 * t11949 / F::new(1536.0) - t2312 * t11953 / F::new(192.0) + t9592 + t11959 - t9598 - t2343 * t11961 / F::new(768.0) - F::new(5.0) / F::new(384.0) * t2343 * t11966 - t2343 * t11970 / F::new(1536.0);
    (t11959, t11961, t11964, t11966, t11970, t11973)
}
