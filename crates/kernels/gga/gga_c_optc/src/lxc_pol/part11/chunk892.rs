//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 892/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk892<F: Float>(t13703: F, t16630: F, t16634: F, t16638: F, t16642: F, t16646: F, t16650: F, t16743: F, t16747: F, t16750: F, t16756: F, t16759: F, t16763: F, t16766: F) -> F {
    let t16856 = F::cast_from(0.29896666666666666667e0_f64) * t13703 + F::new(0.1898925e1) * t16743 - F::cast_from(0.29896666666666666667e0_f64) * t16650 - F::cast_from(0.49293999999999999999e0_f64) * t16747 + F::cast_from(0.16431333333333333333e0_f64) * t16750 + F::cast_from(0.11958666666666666667e1_f64) * t16634 - F::new(0.17938e1) * t16642 - F::cast_from(0.33218518518518518518e0_f64) * t16630 - F::cast_from(0.36514074074074074075e-1_f64) * t16756 - F::cast_from(0.82156666666666666667e-1_f64) * t16759 + F::new(0.17938e1) * t16646 + F::cast_from(0.49293999999999999999e0_f64) * t16763 - F::cast_from(0.82156666666666666668e-1_f64) * t16766 - F::cast_from(0.59793333333333333333e0_f64) * t16638;
    t16856
}
