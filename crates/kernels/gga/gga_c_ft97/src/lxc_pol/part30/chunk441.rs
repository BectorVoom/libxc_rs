//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 441/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk441<F: Float>(t2843: F, t7114: F, t296: F, t6380: F, t6383: F, t7065: F, t7069: F, t7073: F, t7077: F, t7081: F, t7085: F, t7089: F) -> (F, F) {
    let t7115 = t2843 * t7114;
    let t7116 = t296 * t7115;
    let t7124 = t7065 / F::cast_from(4.0_f64) + t6380 + t7069 / F::cast_from(6.0_f64) + t7073 - t7077 / F::cast_from(2.0_f64) + t6383 + t7081 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) * t7085 - t7089;
    (t7116, t7124)
}
