//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 38/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk38<F: Float>(t11: F, t14: F, t17: F, t25: F) -> (F, F, F) {
    let t80 = F::new(0.51785e1) * t14 + F::new(0.905775e0) * t11 + F::new(0.1100325e0) * t17 + F::new(0.1241775e0) * t25;
    let t83 = F::new(1.0) + F::new(0.29608574643216675549e2) / t80;
    let t84 = f64::ln(t83);
    (t80, t83, t84)
}
