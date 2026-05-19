//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1074/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1074<F: Float>(t16490: F, t18149: F, t18155: F, t42943: F, t42948: F, t47293: F, t47297: F, t47299: F, t47301: F, t47303: F, t47307: F, t12339: F, t1820: F, t1821: F, t7899: F) -> (F, F) {
    let t47308 = t18149 + F::new(4.0) / F::new(3.0) * t42943 - t18155 + F::cast_from(0.24311111111111111111e0_f64) * t42948 - t16490 + t47293 + t47297 + t47299 + t47301 - t47303 + t47307;
    let t47315 = F::new(64.0) / F::new(15.0) * t1820 * t1821 * t7899 * t12339;
    (t47308, t47315)
}
