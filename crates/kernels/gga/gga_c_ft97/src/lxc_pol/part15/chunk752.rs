//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 752/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk752<F: Float>(t21027: F, t21031: F, t21036: F, t21040: F, t21044: F, t21048: F, t21052: F, t21056: F, t21059: F, t21062: F, t21064: F, t2265: F, t631: F) -> F {
    let t21066 = t631 * t21027 / F::cast_from(2.0_f64) + t631 * t21031 / F::cast_from(6.0_f64) + F::cast_from(6.0_f64) * t631 * t21036 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t631 * t21040 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t631 * t21044 + F::cast_from(3.0_f64) * t2265 * t21048 + F::cast_from(2.0_f64) * t2265 * t21052 - t2265 * t21056 - t2265 * t21059 + t631 * t21062 - t2265 * t21064;
    t21066
}
