//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 865/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk865<F: Float>(t332: F, t6238: F, t863: F, t3037: F, t339: F, t3184: F, t6484: F, t1114: F, t6701: F, t2119: F, t3039: F, t6710: F) -> (F, F, F, F, F, F) {
    let t8903 = t863 * t6238 * t332;
    let t8913 = t3037 * t339;
    let t8927 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t6484 * t3184;
    let t8928 = t1114 * t6701;
    let t8949 = t3039 * t2119;
    let t8956 = t1114 * t6710;
    (t8903, t8913, t8927, t8928, t8949, t8956)
}
