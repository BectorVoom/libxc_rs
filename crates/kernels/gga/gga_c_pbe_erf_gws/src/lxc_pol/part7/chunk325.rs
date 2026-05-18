//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 325/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk325<F: Float>(t1238: F, t1241: F, t1243: F, t1247: F, t1249: F, t1251: F) -> F {
    let t1253 = -F::new(0.78438333333333333333e0) * t1238 + F::new(0.15687666666666666667e1) * t1241 + F::new(0.68863333333333333333e0) * t1243 + F::new(0.14025833333333333333e0) * t1247 + F::new(0.28051666666666666667e0) * t1249 + F::new(0.17365833333333333333e0) * t1251;
    t1253
}
