//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1418/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1418(t2927: f64, t30657: f64, t11377: f64, t1828: f64, t11406: f64, t11376: f64, t11410: f64, t30570: f64, t1161: f64, t2876: f64, t4512: f64, t11266: f64, t7785: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30682 = t2927 * t30657;
    let t30685 = t11377 * t1828;
    let t30686 = t11406 * t30685;
    let t30689 = t11376 * t30685;
    let t30692 = t11410 * t30685;
    let t30697 = t11406 * t30570;
    let t30703 = t1161 * t4512 * t2876;
    let t30710 = t11266 * t7785;
    (t30682, t30686, t30689, t30692, t30697, t30703, t30710)
}
