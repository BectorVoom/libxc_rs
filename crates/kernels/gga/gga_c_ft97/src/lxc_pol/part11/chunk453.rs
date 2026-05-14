//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 453/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk453<F: Float>(t2579: F, t729: F, t762: F, t1882: F, t726: F, t684: F, t724: F, t773: F, t2413: F, t265: F, t2404: F, t241: F) -> (F, F, F, F, F) {
    let t2581 = t729 * t762 * t2579;
    let t2584 = t1882 * t726;
    let t2587 = t724 * t773 * t684;
    let t2591 = t724 * t265 * t2413;
    let t2594 = t2404 * t241;
    (t2581, t2584, t2587, t2591, t2594)
}
