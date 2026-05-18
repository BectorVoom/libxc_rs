//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 689/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk689<F: Float>(t20138: F, t446: F, t4462: F, t942: F, t1564: F, t4495: F, t925: F, t1558: F, t20022: F, t356: F, t89: F, t20039: F, t447: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20139 = t446 * t20138;
    let t20141 = t4462 * t942;
    let t20142 = t1564 * t20141;
    let t20143 = t446 * t20142;
    let t20145 = t925 * t4495;
    let t20146 = t1564 * t20145;
    let t20147 = t446 * t20146;
    let t20149 = t1558 * t20022;
    let t20151 = t89 * t356 * t20149;
    let t20153 = t447 * t20039;
    (t20139, t20141, t20142, t20143, t20145, t20146, t20147, t20149, t20151, t20153)
}
