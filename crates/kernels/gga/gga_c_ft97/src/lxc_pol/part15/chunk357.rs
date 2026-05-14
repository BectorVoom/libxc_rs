//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 357/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk357<F: Float>(t2347: F, t2440: F, t2360: F, t703: F, t250: F, t251: F, t1771: F, t249: F, t2344: F, t241: F) -> (F, F, F, F, F) {
    let t2441 = t2440 * t2347;
    let t2446 = t703 * t2360;
    let t2475 = 1.0 / t251 / t250;
    let t2481 = 4.0 / 9.0 * t1771 * t249;
    let t2486 = t2344 * t241;
    (t2441, t2446, t2475, t2481, t2486)
}
