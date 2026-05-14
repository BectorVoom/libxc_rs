//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 904/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk904<F: Float>(t1114: F, t6710: F, t2150: F, t3128: F, t6332: F, t2494: F, t5: F, t337: F, t2147: F, t2146: F, t2153: F, t838: F, t863: F, t3142: F, t3172: F, t6484: F) -> (F, F, F, F, F, F, F) {
    let t8956 = t1114 * t6710;
    let t8958 = t8956 * t2150 / 24.0;
    let t8960 = 7.0 / 72.0 * t3128 * t6332;
    let t8961 = t5 * t2494;
    let t8962 = t337 * t8961;
    let t8963 = t2147 * t8962;
    let t8965 = t2146 * t8963 / 24.0;
    let t8967 = t863 * t2153 * t838;
    let t8969 = 7.0 / 72.0 * t8967 * t3142;
    let t8971 = 7.0 / 144.0 * t6484 * t3172;
    (t8958, t8960, t8961, t8962, t8965, t8969, t8971)
}
