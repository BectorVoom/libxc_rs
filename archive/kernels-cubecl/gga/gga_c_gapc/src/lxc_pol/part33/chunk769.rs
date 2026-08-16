//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 769/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk769<F: Float>(t9009: F, t9011: F, t9014: F, t9017: F, t9021: F, t9024: F, t9027: F, t9032: F, t9034: F, t9036: F, t9038: F, t9042: F, t9044: F) -> F {
    let t9046 = -F::cast_from(0.12357942809624928455e-3_f64) * t9009 - F::cast_from(0.18326250058315256483e-6_f64) * t9011 - F::cast_from(0.27801896084645508334e-2_f64) * t9014 + F::cast_from(0.75883739738679928911e-6_f64) * t9017 - F::cast_from(0.13492128925537291361e-5_f64) * t9021 - F::cast_from(0.7588373973867992891e-7_f64) * t9024 + F::cast_from(0.13492128925537291361e-6_f64) * t9027 + F::cast_from(0.7324140771837707598e-5_f64) * t9032 - F::cast_from(0.2318836277704281739e-4_f64) * t9034 + F::cast_from(0.56360603971979070047e-7_f64) * t9036 - F::cast_from(0.10020915386217878654e-6_f64) * t9038 + F::cast_from(0.56275309320814680968e-8_f64) * t9042 + F::cast_from(0.5627530932081468097e-7_f64) * t9044;
    t9046
}
