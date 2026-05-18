//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 338/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk338<F: Float>(t1238: F, t1241: F, t1243: F, t1247: F, t1249: F, t1251: F) -> F {
    let t1314 = -F::new(0.57538888888888888889e0) * t1238 + F::new(0.11507777777777777778e1) * t1241 + F::new(0.40256666666666666667e0) * t1243 + F::new(0.366775e-1) * t1247 + F::new(0.73355e-1) * t1249 + F::new(0.137975e0) * t1251;
    t1314
}
