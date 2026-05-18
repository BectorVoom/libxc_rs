//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 527/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk527<F: Float>(t4129: F, t799: F, t27: F, t89: F, t2653: F, t2655: F, t2658: F, t4032: F, t4035: F, t4039: F, t4042: F, t4046: F, t4049: F, t4054: F, t4059: F) -> (F, F, F) {
    let t4130 = t799 * t4129;
    let t4132 = t89 * t27 * t4130;
    let t4134 = t2653 + t2655 / F::new(54.0) + t2658 / F::new(18.0) + t4032 / F::new(54.0) - t4035 / F::new(27.0) + t4039 / F::new(18.0) + t4042 / F::new(9.0) + t4046 / F::new(9.0) + t4049 / F::new(18.0) + t4054 / F::new(18.0) + t4059 / F::new(3.0) - t4132 / F::new(6.0);
    (t4130, t4132, t4134)
}
