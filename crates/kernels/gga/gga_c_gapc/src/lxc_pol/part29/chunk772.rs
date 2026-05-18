//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 772/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk772<F: Float>(t9009: F, t9011: F, t9014: F, t9017: F, t9021: F, t9024: F, t9027: F, t9032: F, t9034: F, t9036: F, t9038: F, t9042: F, t9044: F) -> F {
    let t9046 = -F::new(0.12357942809624928455e-3) * t9009 - F::new(0.18326250058315256483e-6) * t9011 - F::new(0.27801896084645508334e-2) * t9014 + F::new(0.75883739738679928911e-6) * t9017 - F::new(0.13492128925537291361e-5) * t9021 - F::new(0.7588373973867992891e-7) * t9024 + F::new(0.13492128925537291361e-6) * t9027 + F::new(0.7324140771837707598e-5) * t9032 - F::new(0.2318836277704281739e-4) * t9034 + F::new(0.56360603971979070047e-7) * t9036 - F::new(0.10020915386217878654e-6) * t9038 + F::new(0.56275309320814680968e-8) * t9042 + F::new(0.5627530932081468097e-7) * t9044;
    t9046
}
