//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 799/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk799<F: Float>(t3037: F, t339: F, t3184: F, t6484: F, t1114: F, t6701: F, t2119: F, t3039: F, t6710: F, t3128: F, t6332: F, t2494: F, t5: F, t337: F, t2147: F, t2153: F, t838: F, t863: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8913 = t3037 * t339;
    let t8927 = 7.0 / 72.0 * t6484 * t3184;
    let t8928 = t1114 * t6701;
    let t8949 = t3039 * t2119;
    let t8956 = t1114 * t6710;
    let t8960 = 7.0 / 72.0 * t3128 * t6332;
    let t8961 = t5 * t2494;
    let t8962 = t337 * t8961;
    let t8963 = t2147 * t8962;
    let t8967 = t863 * t2153 * t838;
    (t8913, t8927, t8928, t8949, t8956, t8960, t8962, t8963, t8967)
}
