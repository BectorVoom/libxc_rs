//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 797/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk797<F: Float>(t30268: F, t7357: F, t1181: F, t14575: F, t599: F, t7346: F, t1983: F, t30262: F, t4210: F, t7586: F, t1170: F, t8462: F, t12816: F, t604: F, t7493: F, t7685: F, t957: F) -> (F, F, F, F, F, F) {
    let t30269 = t30268 * t7357;
    let t30273 = t7346 * t1181 * t599 * t14575;
    let t30280 = t30262 * t7586 * t1983 * t4210;
    let t30282 = t1170 * t8462;
    let t30297 = t7493 * t1181 * t604 * t12816;
    let t30301 = t7685 * t957;
    (t30269, t30273, t30280, t30282, t30297, t30301)
}
