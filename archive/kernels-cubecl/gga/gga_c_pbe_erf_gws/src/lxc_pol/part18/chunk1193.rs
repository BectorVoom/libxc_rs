//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1193/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1193<F: Float>(t15345: F, t3965: F, t14469: F, t14657: F, t1192: F, t3721: F, t2409: F, t9296: F, t3825: F, t3990: F, t3991: F, t3989: F) -> (F, F, F, F, F, F) {
    let t15346 = t3965 * t15345;
    let t15348 = t14657 * t14469;
    let t15351 = t1192 * t3721;
    let t15353 = t2409 * t9296 * t15351;
    let t15357 = t3990 * t3991 * t3825;
    let t15358 = t3989 * t15357;
    (t15346, t15348, t15351, t15353, t15357, t15358)
}
