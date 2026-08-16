//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 816/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk816<F: Float>(t157: F, t9224: F, t160: F, t7763: F, t7800: F, t1570: F, t586: F, t1557: F, t2: F, t1985: F, t2097: F, t597: F) -> (F, F, F, F, F, F, F, F) {
    let t12723 = t9224 * t157;
    let t12724 = t160 * t7763;
    let t12746 = t160 * t7800;
    let t12791 = t586 * t1570;
    let t12796 = t586 * t1557;
    let t12823 = t9224 * t2;
    let t12968 = t1985 * t157;
    let t12982 = t2097 * t597;
    (t12723, t12724, t12746, t12791, t12796, t12823, t12968, t12982)
}
