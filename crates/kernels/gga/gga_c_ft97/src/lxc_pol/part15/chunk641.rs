//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 641/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk641<F: Float>(t20022: F, t7764: F, t7761: F, t89: F, t4454: F, t942: F, t7793: F, t446: F, t7801: F, t1555: F, t1866: F, t20031: F, t4462: F, t1564: F, t4495: F, t925: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t20124 = t7764 * t20022;
    let t20126 = t89 * t7761 * t20124;
    let t20130 = t4454 * t942;
    let t20131 = t7793 * t20130;
    let t20132 = t446 * t20131;
    let t20134 = t7801 * t20022;
    let t20136 = t89 * t1555 * t20134;
    let t20138 = t1866 * t20031;
    let t20139 = t446 * t20138;
    let t20141 = t4462 * t942;
    let t20142 = t1564 * t20141;
    let t20143 = t446 * t20142;
    let t20145 = t925 * t4495;
    (t20124, t20126, t20130, t20131, t20132, t20134, t20136, t20138, t20139, t20141, t20142, t20143, t20145)
}
