//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1040/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1040<F: Float>(t824: F, t9263: F, t905: F, t2319: F, t3299: F, t2271: F, t3038: F, t2255: F, t3258: F, t6573: F, t367: F, t6553: F, t899: F) -> (F, F, F, F, F, F, F) {
    let t9410 = t9263 * t824;
    let t9411 = t905 * t9410;
    let t9415 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t2319 * t3299;
    let t9416 = t3038 * t2271;
    let t9417 = t905 * t9416;
    let t9421 = t2255 * t3258 * t6573;
    let t9425 = t899 * t6553 * t367;
    (t9410, t9411, t9415, t9416, t9417, t9421, t9425)
}
