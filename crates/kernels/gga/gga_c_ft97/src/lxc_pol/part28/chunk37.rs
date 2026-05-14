//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 37/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk37<F: Float>(t52: F, t54: F, t58: F, t41: F, t42: F, t47: F) -> (F, F, F) {
    let t60 = t52 * t54 * t58;
    let t61 = 0.55569193573523559258e-3 * t60;
    let t62 = 1.0 + 0.45058854638888888889e-1 * t41 * t42 * t47 + t61;
    (t60, t61, t62)
}
