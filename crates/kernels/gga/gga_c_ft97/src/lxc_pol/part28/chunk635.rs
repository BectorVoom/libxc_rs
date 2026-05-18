//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 635/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk635<F: Float>(t5743: F, t942: F, t452: F, t488: F, t110: F, t1871: F, t25990: F, t492: F, t6454: F, t11902: F, t5631: F, t23339: F, t3214: F) -> (F, F, F, F, F, F, F) {
    let t26145 = t5743 * t942;
    let t26147 = t452 * t488 * t26145;
    let t26151 = t1871 * t110 * t25990;
    let t26154 = t6454 * t492;
    let t26156 = t452 * t488 * t26154;
    let t26159 = t11902 * t5631;
    let t26162 = t23339 * t3214;
    (t26145, t26147, t26151, t26154, t26156, t26159, t26162)
}
