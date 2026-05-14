//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 647/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk647<F: Float>(t1903: F, t720: F, t254: F, t542: F, t252: F, t1907: F, t723: F, t245: F, t713: F, t1802: F, t610: F, t1866: F, t1885: F, t587: F, t1697: F, t212: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5384 = 2.0 / 9.0 * t720 * t1903;
    let t5385 = t254 * t542;
    let t5387 = 8.0 / 81.0 * t252 * t5385;
    let t5388 = t1907 * t723;
    let t5390 = t245 * t713;
    let t5393 = t1802 * t610;
    let t5394 = t5393 * t1866;
    let t5395 = t1885 * t5394;
    let t5397 = 4.0 / 5.0 * t587 * t5395;
    let t5399 = 1.0 / t212 / t1697;
    (t5384, t5385, t5387, t5388, t5390, t5393, t5394, t5395, t5397, t5399)
}
