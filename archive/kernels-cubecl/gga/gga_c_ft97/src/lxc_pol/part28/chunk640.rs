//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 640/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk640<F: Float>(t26198: F, t8557: F, t23265: F, t3113: F, t11854: F, t379: F, t447: F, t6564: F, t3052: F, t5717: F, t1909: F, t6534: F, t8506: F) -> (F, F, F, F, F, F, F) {
    let t26199 = t8557 * t26198;
    let t26202 = t23265 * t3113;
    let t26203 = t11854 * t26202;
    let t26207 = t447 * t6564 * t379;
    let t26210 = t5717 * t3052;
    let t26211 = t1909 * t26210;
    let t26214 = t8506 * t6534;
    (t26199, t26202, t26203, t26207, t26210, t26211, t26214)
}
