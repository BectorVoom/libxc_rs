//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 736/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk736<F: Float>(t9871: F, t9973: F, t241: F, t258: F, t259: F, t89: F, t9555: F, t2459: F, t684: F, t2599: F, t2413: F, t724: F, t773: F) -> (F, F, F, F, F, F, F) {
    let t9974 = t9871 + t9973;
    let t9976 = t241 * t9974 * t258;
    let t9982 = F::new(28.0) / F::new(81.0) * t89 * t9555 * t259;
    let t9983 = t258 * t2459;
    let t9984 = t9983 * t684;
    let t9985 = t2599 * t9984;
    let t9989 = t724 * t773 * t2413;
    (t9974, t9976, t9982, t9983, t9984, t9985, t9989)
}
