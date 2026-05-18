//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 867/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk867<F: Float>(t3128: F, t6332: F, t2494: F, t5: F, t337: F, t2147: F, t2153: F, t838: F, t863: F, t3142: F, t3172: F, t6484: F) -> (F, F, F, F, F) {
    let t8960 = F::new(7.0) / F::new(72.0) * t3128 * t6332;
    let t8961 = t5 * t2494;
    let t8962 = t337 * t8961;
    let t8963 = t2147 * t8962;
    let t8967 = t863 * t2153 * t838;
    let t8969 = F::new(7.0) / F::new(72.0) * t8967 * t3142;
    let t8971 = F::new(7.0) / F::new(144.0) * t6484 * t3172;
    (t8960, t8962, t8963, t8969, t8971)
}
