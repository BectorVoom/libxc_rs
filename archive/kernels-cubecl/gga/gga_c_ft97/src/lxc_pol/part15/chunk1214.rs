//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1214/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1214<F: Float>(t43212: F, t52453: F, t66197: F, t66202: F, t66221: F, t80029: F, t80031: F, t88740: F, t88744: F, t88747: F, t88751: F, t88754: F, t88761: F, t88769: F) -> F {
    let t91290 = F::cast_from(0.38514888888888888888e0_f64) * t80029 - F::cast_from(0.11554466666666666666e1_f64) * t80031 + F::cast_from(0.11554466666666666666e1_f64) * t88761 - F::cast_from(0.9628722222222222222e0_f64) * t88769 + F::cast_from(0.34663399999999999999e1_f64) * t88740 - F::cast_from(0.38514888888888888888e0_f64) * t88747 - F::cast_from(0.51995099999999999998e1_f64) * t88754 + F::cast_from(0.59912049382716049381e0_f64) * t52453 + t43212 - F::cast_from(0.25676592592592592592e0_f64) * t66197 - F::cast_from(0.38514888888888888888e0_f64) * t66202 + F::cast_from(0.77029777777777777776e0_f64) * t66221 - F::cast_from(0.28886166666666666666e0_f64) * t88744 + F::cast_from(0.34663399999999999999e1_f64) * t88751;
    t91290
}
