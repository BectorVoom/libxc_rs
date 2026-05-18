//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 631/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk631<F: Float>(t23048: F, t23055: F, t25958: F, t25962: F, t25966: F, t25970: F, t25973: F, t25976: F, t25979: F, t25983: F, t25988: F, t25993: F) -> F {
    let t26102 = -F::new(2.0) / F::new(3.0) * t23048 + t25958 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t25962 - F::new(6.0) * t25966 - t23055 / F::new(18.0) - F::new(2.0) / F::new(3.0) * t25970 - F::new(2.0) / F::new(3.0) * t25973 - F::new(2.0) / F::new(3.0) * t25976 + F::new(2.0) / F::new(9.0) * t25979 - t25983 / F::new(12.0) - t25988 / F::new(12.0) + t25993;
    t26102
}
