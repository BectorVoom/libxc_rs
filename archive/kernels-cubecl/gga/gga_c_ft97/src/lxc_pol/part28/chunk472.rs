//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 472/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk472<F: Float>(t1286: F, t1310: F, t7162: F, t7168: F, t7214: F, t7218: F, t7270: F, t7275: F, t7282: F, t7286: F, t7288: F, t88: F) -> F {
    let t7293 = t7162 * t1310 / F::cast_from(6.0_f64) - t1286 * t7168 / F::cast_from(3.0_f64) + t1286 * t7214 / F::cast_from(6.0_f64) + t1286 * t7218 / F::cast_from(3.0_f64) - t88 * t7286 + F::cast_from(2.0_f64) * t7288 - F::cast_from(4.0_f64) * t7270 + F::cast_from(4.0_f64) * t7275 - F::cast_from(2.0_f64) * t7282;
    t7293
}
