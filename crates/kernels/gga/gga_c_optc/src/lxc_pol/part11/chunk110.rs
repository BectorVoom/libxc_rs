//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 110/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk110<F: Float>(t214: F, t217: F, t220: F, t226: F) -> (F, F, F) {
    let t261 = F::cast_from(0.51785e1_f64) * t217 + F::cast_from(0.905775e0_f64) * t214 + F::cast_from(0.1100325e0_f64) * t220 + F::cast_from(0.1241775e0_f64) * t226;
    let t264 = F::cast_from(1.0_f64) + F::cast_from(0.29608574643216675549e2_f64) / t261;
    let t265 = F::ln(t264);
    (t261, t264, t265)
}
