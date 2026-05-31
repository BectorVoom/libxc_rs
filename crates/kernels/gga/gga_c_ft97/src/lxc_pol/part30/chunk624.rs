//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 624/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk624<F: Float>(t27844: F, t27848: F, t27853: F, t27858: F, t27861: F, t27864: F, t27867: F, t27870: F, t27873: F, t27876: F, t27881: F, t27885: F) -> F {
    let t28095 = t27844 / F::cast_from(3.0_f64) + t27848 / F::cast_from(12.0_f64) + t27853 / F::cast_from(12.0_f64) + t27858 / F::cast_from(12.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t27861 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t27864 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t27867 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t27870 - t27873 / F::cast_from(36.0_f64) - t27876 / F::cast_from(9.0_f64) + t27881 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t27885;
    t28095
}
