//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 624/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk624<F: Float>(t1928: F, t3515: F, t3517: F, t3521: F, t3525: F, t3529: F, t3533: F, t3537: F, t3538: F, t3557: F, t3559: F, t3561: F, t3566: F, t3591: F, t3592: F, t3606: F) -> (F,) {
    let t3607 = t1928 - t3515 + t3517 + t3521 + t3525 + t3529 + t3533 - t3537 + t3538 - t3557 - t3559 + t3561 + t3566;
    let t3609 = t3591 + t3592 + t3606 + t3607;
    (t3609,)
}
