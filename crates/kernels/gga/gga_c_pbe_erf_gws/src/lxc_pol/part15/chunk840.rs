//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 840/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk840<F: Float>(t4996: F, t4987: F, t7026: F, t7031: F, t7033: F, t7038: F, t7042: F, t7045: F, t7047: F, t7054: F, t7060: F, t7067: F, t7072: F, t7074: F, t7075: F, t7077: F, t7079: F) -> (F, F) {
    let t7080 = F::new(16.0) / F::new(135.0) * t4996;
    let t7081 = -t7026 + t7031 - t7033 - t7038 + t7042 - t7045 + t7047 + t7054 - t7060 + t7067 - t7072 + t7074 + F::new(4.0) / F::new(9.0) * t7075 + t7077 - F::new(2.0) / F::new(45.0) * t4987 - t7079 - t7080;
    (t7080, t7081)
}
