//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 992/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk992<F: Float>(t7400: F, t9438: F, t7354: F, t8232: F, t1882: F, t33012: F, t33207: F, t7359: F, t33041: F, t8392: F, t7409: F, t33184: F) -> (F, F, F, F, F, F, F, F) {
    let t139757 = t9438 * t7400;
    let t139767 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t8232 * t7354;
    let t139791 = t1882 * t33012;
    let t139808 = t1882 * t33207;
    let t139811 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t8232 * t7359;
    let t139820 = t8392 * t33041;
    let t139823 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t8232 * t7409;
    let t139888 = t1882 * t33184;
    (t139757, t139767, t139791, t139808, t139811, t139820, t139823, t139888)
}
