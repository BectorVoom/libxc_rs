//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 25/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk25<F: Float>(t29: F, t25: F) -> (F, F, F) {
    let t30 = F::powf(t29, F::cast_from(1.0_f64) / F::cast_from(5.0_f64));
    let t31 = t30 * t30;
    let t32 = t31 * t31;
    let t35 = F::exp(-F::cast_from(0.2081897e-1_f64) * t25 * t32);
    (t30, t31, t35)
}
