//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1260/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1260<F: Float>(t13953: F, t14648: F, t13972: F, t14684: F, t14473: F, t840: F, t14579: F, t14576: F, t2376: F, t829: F, t830: F, t14608: F) -> (F, F, F, F, F, F) {
    let t54429 = t13953 * t14648;
    let t54430 = F::new(7.0) / F::new(144.0) * t54429;
    let t54463 = t13972 * t14684;
    let t54464 = F::new(7.0) / F::new(1152.0) * t54463;
    let t54480 = F::new(7.0) / F::new(144.0) * t840 * t14473;
    let t54482 = F::new(7.0) / F::new(144.0) * t840 * t14579;
    let t54486 = t2376 * t14576;
    let t54488 = t829 * t830 * t54486;
    let t54491 = t13972 * t14608;
    (t54430, t54464, t54480, t54482, t54488, t54491)
}
