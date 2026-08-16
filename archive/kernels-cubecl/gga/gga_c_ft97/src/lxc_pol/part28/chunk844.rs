//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 844/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk844<F: Float>(t32446: F, t32449: F, t32453: F, t34413: F, t34418: F, t34485: F, t34489: F, t34493: F, t34497: F, t34501: F, t34505: F, t34509: F) -> F {
    let t34510 = t32446 + t34413 / F::cast_from(6.0_f64) + t34418 - t34485 / F::cast_from(2.0_f64) - t32449 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t34489 - F::cast_from(6.0_f64) * t34493 + F::cast_from(4.0_f64) * t34497 + t32453 + t34501 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) * t34505 - t34509;
    t34510
}
