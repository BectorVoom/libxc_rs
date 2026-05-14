//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 726/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk726<F: Float>(t3123: F, t8888: F, t9009: F, t9011: F, t9014: F, t9017: F, t9021: F, t9024: F, t9027: F, t9032: F, t9034: F, t9036: F, t9038: F, t9042: F, t1448: F, t3116: F) -> (F, F, F) {
    let t9044 = t8888 * t3123;
    let t9046 = -0.12357942809624928455e-3 * t9009 - 0.18326250058315256483e-6 * t9011 - 0.27801896084645508334e-2 * t9014 + 0.75883739738679928911e-6 * t9017 - 0.13492128925537291361e-5 * t9021 - 0.7588373973867992891e-7 * t9024 + 0.13492128925537291361e-6 * t9027 + 0.7324140771837707598e-5 * t9032 - 0.2318836277704281739e-4 * t9034 + 0.56360603971979070047e-7 * t9036 - 0.10020915386217878654e-6 * t9038 + 0.56275309320814680968e-8 * t9042 + 0.5627530932081468097e-7 * t9044;
    let t9047 = t1448 * t3116;
    (t9044, t9046, t9047)
}
