//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 160/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk160<F: Float>(t374: F, t930: F, t423: F, t920: F, t420: F, t419: F, t417: F) -> (F, F, F, F, F) {
    let t931 = t374 * t930;
    let t934 = t423 * t920;
    let t935 = t420 * t934;
    let t936 = t419 * t935;
    let t938 = t417 + F::cast_from(0.6384360837962962963e-2_f64) * t936;
    (t931, t934, t935, t936, t938)
}
