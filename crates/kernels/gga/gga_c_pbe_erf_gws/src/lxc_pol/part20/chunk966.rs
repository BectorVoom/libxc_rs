//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 966/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk966<F: Float>(t10898: F, t184: F, t997: F, t7171: F, t5465: F, t10888: F, t10890: F, t10894: F, t10895: F, t10897: F, t5418: F, t5423: F, t5429: F, t5430: F, t5433: F, t5436: F, t5437: F, t5443: F, t7775: F) -> (F, F, F, F) {
    let t10899 = t10898 * t184;
    let t10901 = F::new(8.0) / F::new(15.0) * t10899 * t997;
    let t10903 = F::new(8.0) / F::new(15.0) * t7171 * t997;
    let t10904 = F::new(8.0) / F::new(405.0) * t5465;
    let t10905 = F::new(0.12155555555555555555e0) * t5418 + t5423 + t5429 + F::new(4.0) / F::new(9.0) * t5430 + t5433 + t5436 - F::new(2.0) / F::new(27.0) * t5437 - t5443 + t10888 + t10890 + t10894 + t7775 - t10895 - t10897 + t10901 + t10903 - t10904;
    (t10901, t10903, t10904, t10905)
}
