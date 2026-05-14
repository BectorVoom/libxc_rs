//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 694/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk694<F: Float>(t4442: F, t847: F, t2366: F, t2387: F, t833: F, t2395: F, t814: F, t829: F, t830: F, t2100: F, t831: F, t2228: F, t840: F, t2367: F, t2373: F, t2306: F, t2365: F) -> (F, F, F, F, F, F, F, F) {
    let t4443 = t4442 * t847;
    let t4453 = t2387 * t2366;
    let t4454 = t4453 * t833;
    let t4459 = t829 * t830 * t2395 * t814;
    let t4464 = t829 * t830 * t831 * t2100;
    let t4467 = t840 * t2228;
    let t4469 = t2367 * t2373;
    let t4473 = t2306 * t2365;
    (t4443, t4453, t4454, t4459, t4464, t4467, t4469, t4473)
}
