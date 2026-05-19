//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 861/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk861<F: Float>(t5907: F, t679: F, t5904: F, t666: F, t16553: F, t16556: F, t16557: F, t16561: F, t16566: F, t16567: F, t16572: F, t16574: F, t16580: F, t225: F, t231: F) -> F {
    let t16584 = t5907 * t679;
    let t16586 = t666 * t5904;
    let t16588 = t16553 + t16556 + F::cast_from(0.86568330898918747016e0_f64) * t16557 - t16561 + t16566 + F::cast_from(0.13418091289332405787e0_f64) * t16567 + t16572 + t16574 + F::new(4.0) / F::new(3.0) * t16580 * t225 * t231 + F::new(16.0) / F::new(3.0) * t16584 + F::new(16.0) / F::new(3.0) * t16586;
    t16588
}
