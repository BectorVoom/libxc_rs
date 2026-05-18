//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1069/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1069<F: Float>(t144829: F, t144832: F, t144836: F, t144840: F, t144844: F, t144848: F, t144851: F, t144855: F, t144859: F, t144863: F, t144866: F, t144870: F, t144874: F, t144878: F, t144882: F, t144886: F) -> F {
    let t145807 = -t144829 / F::new(36.0) - t144832 / F::new(3.0) - F::new(20.0) / F::new(3.0) * t144836 + F::new(8.0) / F::new(3.0) * t144840 - t144844 / F::new(36.0) + t144848 / F::new(3.0) - t144851 / F::new(9.0) - t144855 / F::new(9.0) - F::new(2.0) / F::new(27.0) * t144859 - t144863 / F::new(3.0) + F::new(4.0) * t144866 - F::new(2.0) * t144870 - F::new(2.0) / F::new(9.0) * t144874 + t144878 / F::new(2.0) + t144882 / F::new(4.0) + t144886 / F::new(2.0);
    t145807
}
