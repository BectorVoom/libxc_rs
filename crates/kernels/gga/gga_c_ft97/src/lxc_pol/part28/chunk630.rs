//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 630/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk630<F: Float>(t22980: F, t22991: F, t23016: F, t23029: F, t23038: F, t25926: F, t25931: F, t25935: F, t25940: F, t25944: F, t25946: F, t25948: F) -> F {
    let t26089 = -t25926 / F::new(3.0) + t25931 / F::new(9.0) - t25935 / F::new(3.0) - t22980 / F::new(3.0) - t22991 / F::new(9.0) + t25940 / F::new(3.0) + t25944 / F::new(3.0) - t25946 / F::new(9.0) - t25948 / F::new(18.0) - t23016 / F::new(12.0) + t23029 / F::new(6.0) - t23038;
    t26089
}
