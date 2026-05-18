//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 847/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk847<F: Float>(t32114: F, t32332: F, t32349: F, t34413: F, t34418: F, t34485: F, t34489: F, t34493: F, t34497: F, t34501: F, t34505: F, t34509: F) -> F {
    let t34534 = t32114 + t34413 / F::new(18.0) + t34418 / F::new(3.0) - t34485 / F::new(6.0) - t32332 - F::new(2.0) / F::new(9.0) * t34489 - F::new(2.0) * t34493 + F::new(4.0) / F::new(3.0) * t34497 + t32349 + t34501 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t34505 - t34509 / F::new(3.0);
    t34534
}
