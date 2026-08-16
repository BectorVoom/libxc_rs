//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1416/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1416(t1539: f64, t5471: f64, t1304: f64, t3663: f64, t1129: f64, t4489: f64, t2875: f64, t30603: f64, t1123: f64, t1144: f64, t1149: f64, t26727: f64, t26883: f64, t26886: f64, t2927: f64, t30574: f64, t30578: f64, t30586: f64, t30600: f64, t9636: f64, t9646: f64, t9650: f64, t9657: f64, t9660: f64) -> (f64, f64, f64, f64) {
    let t30611 = t5471 * t1539;
    let t30615 = t3663 * t1304;
    let t30616 = t4489 * t1129;
    let t30617 = t30615 * t30616;
    let t30620 = t2875 * t30603;
    let t30641 = t4489 * t1123;
    let t30642 = t30615 * t30641;
    let t30648 = -1792.0_f64 / 27.0_f64 * t2875 * t30611 * t9657 - 11200.0_f64 / 27.0_f64 * t26883 * t30617 - 896.0_f64 / 9.0_f64 * t30620 * t9660 - 11200.0_f64 / 27.0_f64 * t30620 * t9636 + 12800.0_f64 / 243.0_f64 * t1149 * t30574 * t30578 + 12800.0_f64 / 243.0_f64 * t1144 * t30574 * t30578 + 6400.0_f64 / 81.0_f64 * t1144 * t26727 * t30586 - 3200.0_f64 / 81.0_f64 * t26886 * t30617 + 256.0_f64 / 27.0_f64 * t30600 * t9660 + 512.0_f64 / 81.0_f64 * t2927 * t30611 * t9646 + 3200.0_f64 / 81.0_f64 * t26886 * t30642 + 256.0_f64 / 27.0_f64 * t2927 * t30603 * t9650;
    (t30611, t30617, t30642, t30648)
}
