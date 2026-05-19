//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 511/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk511<F: Float>(t2938: F, t2939: F, t898: F, t2400: F, t2402: F, t2407: F, t2411: F, t2415: F, t2698: F, t2701: F) -> (F, F) {
    let t2941 = t898 * t2938 * t2939;
    let t2946 = F::cast_from(0.19257444444444444444e0_f64) * t2400;
    let t2951 = -F::new(0.117377e0) * t2698 + F::new(0.234754e0) * t2701 + t2946 + F::cast_from(0.9628722222222222222e-1_f64) * t2402 - F::cast_from(0.9628722222222222222e-1_f64) * t2407 + F::cast_from(0.28886166666666666666e0_f64) * t2411 - F::cast_from(0.14443083333333333333e0_f64) * t2415;
    (t2941, t2951)
}
