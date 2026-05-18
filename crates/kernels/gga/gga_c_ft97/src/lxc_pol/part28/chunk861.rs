//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 861/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk861<F: Float>(t1901: F, t28: F, t32589: F, t34696: F, t34700: F, t34703: F, t34707: F, t34710: F, t34714: F, t34718: F, t34722: F, t34726: F, t34729: F, t34732: F, t446: F, t89: F) -> F {
    let t34735 = t89 * t28 * t34696 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t1901 * t34700 + F::new(4.0) / F::new(3.0) * t446 * t34703 + t1901 * t34707 / F::new(9.0) - t32589 - t446 * t34710 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t446 * t34714 + F::new(2.0) / F::new(3.0) * t446 * t34718 - F::new(2.0) / F::new(3.0) * t446 * t34722 - F::new(4.0) / F::new(3.0) * t1901 * t34726 - t446 * t34729 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t446 * t34732;
    t34735
}
