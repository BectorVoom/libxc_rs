//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 890/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk890<F: Float>(t338: F, t3722: F, t892: F, t2409: F, t3212: F, t8589: F, t3060: F, t8713: F, t9283: F, t3724: F, t840: F, t1161: F) -> (F, F, F, F, F, F) {
    let t9865 = t338 * t892 * t3722;
    let t9869 = t2409 * t8589 * t3212;
    let t9872 = t8713 * t3060;
    let t9873 = t9283 * t9872;
    let t9879 = t840 * t3724;
    let t9883 = t8589 * t1161;
    (t9865, t9869, t9872, t9873, t9879, t9883)
}
