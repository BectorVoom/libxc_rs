//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 720/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk720<F: Float>(t376: F, t814: F, t810: F, t353: F, t4386: F, t2082: F, t322: F, t816: F, t2352: F, t2376: F, t829: F, t830: F) -> (F, F, F, F, F, F) {
    let t4387 = t376 * t814;
    let t4388 = t4387 * t810;
    let t4389 = t353 * t4388;
    let t4390 = t4386 * t4389;
    let t4394 = F::new(1.0) / t2082 / t322;
    let t4395 = t4394 * t816;
    let t4400 = t2376 * t2352;
    let t4402 = t829 * t830 * t4400;
    (t4387, t4390, t4394, t4395, t4400, t4402)
}
