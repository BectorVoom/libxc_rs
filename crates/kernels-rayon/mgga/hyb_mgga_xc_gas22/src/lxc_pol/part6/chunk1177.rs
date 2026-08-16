//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1177/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1177(t7001: f64, t974: f64, t2530: f64, t2559: f64, t21503: f64, t378: f64, t2569: f64, t2598: f64, t6992: f64, t993: f64, t21601: f64, t2593: f64, t6996: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21679 = t974 * t7001;
    let t21700 = t2530 * t2559;
    let t21715 = t378 * t21503;
    let t21721 = t2569 * t2598;
    let t21726 = t993 * t6992;
    let t21729 = t378 * t21601;
    let t21770 = t2593 * t6996;
    (t21679, t21700, t21715, t21721, t21726, t21729, t21770)
}
