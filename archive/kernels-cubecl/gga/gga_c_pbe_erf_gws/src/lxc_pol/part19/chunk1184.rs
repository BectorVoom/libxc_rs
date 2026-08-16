//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1184/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1184<F: Float>(t2409: F, t9897: F, t3965: F, t9818: F, t14121: F, t12136: F, t3959: F, t9888: F, t353: F, t859: F, t9914: F, t14733: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15314 = t2409 * t9897;
    let t15315 = t3965 * t15314;
    let t15317 = t2409 * t9818;
    let t15318 = t14121 * t15317;
    let t15331 = t2409 * t12136;
    let t15332 = t3959 * t15331;
    let t15334 = t2409 * t9888;
    let t15335 = t3959 * t15334;
    let t15342 = t859 * t353 * t9914;
    let t15343 = t14733 * t15342;
    (t15314, t15315, t15317, t15318, t15331, t15332, t15334, t15335, t15342, t15343)
}
