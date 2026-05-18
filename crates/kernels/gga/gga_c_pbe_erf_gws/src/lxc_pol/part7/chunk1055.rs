//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1055/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1055<F: Float>(t1696: F, t52: F, t16973: F, t1413: F, t1416: F, t4366: F, t1528: F, t16986: F, t4370: F, t4373: F, t16978: F, t478: F) -> (F, F, F, F, F) {
    let t19071 = F::new(1.0) / t52 / t1696;
    let t19072 = t19071 * t16973;
    let t19075 = t4366 * t1413 * t1416;
    let t19077 = t1528 * t16986;
    let t19079 = t4370 * t4373;
    let t19081 = t478 * t16978;
    (t19072, t19075, t19077, t19079, t19081)
}
