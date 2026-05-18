//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 850/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk850<F: Float>(t7194: F, t7195: F, t1620: F, t2591: F, t649: F, t2705: F, t661: F, t639: F, t1697: F, t34: F, t422: F, t1639: F, t331: F) -> (F, F, F, F, F) {
    let t7196 = t7194 * t7195;
    let t7198 = F::new(32.0) / F::new(45.0) * t1620 * t7196;
    let t7199 = t2591 * t649;
    let t7200 = t2705 * t661;
    let t7201 = t7199 * t7200;
    let t7203 = F::new(16.0) / F::new(45.0) * t639 * t7201;
    let t7204 = t1697 * t34;
    let t7205 = t7204 * t422;
    let t7206 = t7194 * t7205;
    let t7208 = F::new(32.0) / F::new(45.0) * t639 * t7206;
    let t7209 = t331 * t1639;
    (t7198, t7203, t7205, t7208, t7209)
}
