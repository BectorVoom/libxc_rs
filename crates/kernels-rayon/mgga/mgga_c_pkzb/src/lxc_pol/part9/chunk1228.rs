//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1228/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1228(t21359: f64, t2887: f64, t68: f64, t7593: f64, t7597: f64, t7586: f64, t7589: f64, t7601: f64, t1126: f64, t17780: f64, t18139: f64, t1843: f64, t1885: f64, t2888: f64, t2889: f64, t5537: f64, t5635: f64, t655: f64, t7350: f64, t7592: f64, t7787: f64, t779: f64, t7796: f64) -> f64 {
    let t21360 = t21359 / 72.0_f64;
    let t21362 = t2887 * t68 * t7593;
    let t21365 = t2887 * t68 * t7597;
    let t21376 = t7586 * t7589;
    let t21387 = t2887 * t68 * t7601;
    let t21394 = -0.53100265402527852012e-1_f64 * t18139 * t1126 - t21360 + t21362 / 24.0_f64 + t21365 / 48.0_f64 + t2887 * t2888 * t779 * t7350 * t655 / 16.0_f64 + t2887 * t2888 * t7592 * t1843 / 16.0_f64 - t21376 / 9.0_f64 + t2887 * t2888 * t2889 * t5537 / 48.0_f64 + t2887 * t2888 * t7796 * t5635 / 4.0_f64 - t21387 / 16.0_f64 - 3.0_f64 / 16.0_f64 * t2887 * t2888 * t7787 * t1885 - 0.17149607247227894789e-2_f64 * t17780;
    t21394
}
