//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 797/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk797<F: Float>(t1181: F, t14575: F, t599: F, t7346: F, t1983: F, t30262: F, t4210: F, t7586: F, t1170: F, t8462: F, t2028: F, t7599: F, t2048: F, t2052: F, t7600: F, t154: F, t360: F, t7322: F, t7326: F) -> (F, F, F, F, F, F, F) {
    let t30273 = t7346 * t1181 * t599 * t14575;
    let t30280 = t30262 * t7586 * t1983 * t4210;
    let t30282 = t1170 * t8462;
    let t30307 = t7599 * t2028;
    let t30308 = t30307 * t2048;
    let t30310 = t7600 * t2052;
    let t30314 = t7322 * t154 * t360 * t7326;
    (t30273, t30280, t30282, t30307, t30308, t30310, t30314)
}
