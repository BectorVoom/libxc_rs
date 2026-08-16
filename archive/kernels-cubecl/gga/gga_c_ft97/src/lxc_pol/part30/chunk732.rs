//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 732/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk732<F: Float>(t17839: F, t218: F, t679: F, t689: F, t6028: F, t7853: F, t6018: F, t9: F, t17836: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t33356 = t17839 * sigma2;
    let t33357 = t218 * t679;
    let t33359 = t33356 * t33357 * t689;
    let t33362 = t7853 * t6028;
    let t33365 = t6018 * t9;
    let t33366 = t17836 * t33365;
    (t33356, t33357, t33359, t33362, t33365, t33366)
}
