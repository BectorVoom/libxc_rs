//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1526/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1526<F: Float>(t10326: F, t36: F, t70: F, t2259: F, t627: F, t2291: F, t607: F, t363: F, t41: F, t46: F, t47: F, t2251: F, t606: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t10327 = t36 * t10326;
    let t10328 = t10327 * t70;
    let t10331 = t2259 * t627;
    let t10336 = t607 * t2291;
    let t10344 = F::cast_from(1.0_f64) / t41 / t363;
    let t10345 = sigma0 * t10344;
    let t10355 = F::cast_from(1.0_f64) / t47 / t46;
    let t10356 = t2251 * t606;
    (t10327, t10328, t10331, t10336, t10345, t10355, t10356)
}
