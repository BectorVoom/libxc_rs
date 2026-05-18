//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 627/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk627<F: Float>(t1076: F, t532: F, t1342: F, t1345: F, t1349: F, t1360: F, t1386: F, t1388: F, t1389: F, t145: F, t169: F, t242: F, t2848: F, t2996: F, t2998: F, t3003: F) -> F {
    let t3007 = t532 * t1076;
    let t3011 = -t1342 + F::new(0.53059442957798955452e-1) * t1345 + t1349 + F::new(0.53059442957798955452e-1) * t2996 - F::new(0.31835665774679373271e-1) * t169 * t2998 * t242 - F::new(0.31835665774679373271e-1) * t3003 - F::new(0.31835665774679373271e-1) * t1360 - t1386 + t1388 - F::new(0.1066501354843587606e0) * t1389 - F::new(0.1066501354843587606e0) * t3007 + F::new(0.533250677421793803e-1) * t145 * t2848;
    t3011
}
