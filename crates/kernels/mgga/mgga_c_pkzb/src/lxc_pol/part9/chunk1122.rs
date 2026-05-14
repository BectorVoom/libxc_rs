//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1122/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1122<F: Float>(t2887: F, t68: F, t7597: F, t7586: F, t7589: F, t7601: F, t1126: F, t17780: F, t18139: F, t1843: F, t1885: F, t21360: F, t21362: F, t2888: F, t2889: F, t5537: F, t5635: F, t655: F, t7350: F, t7592: F, t7787: F, t779: F, t7796: F) -> (F,) {
    let t21365 = t2887 * t68 * t7597;
    let t21376 = t7586 * t7589;
    let t21387 = t2887 * t68 * t7601;
    let t21394 = -0.53100265402527852012e-1 * t18139 * t1126 - t21360 + t21362 / 24.0 + t21365 / 48.0 + t2887 * t2888 * t779 * t7350 * t655 / 16.0 + t2887 * t2888 * t7592 * t1843 / 16.0 - t21376 / 9.0 + t2887 * t2888 * t2889 * t5537 / 48.0 + t2887 * t2888 * t7796 * t5635 / 4.0 - t21387 / 16.0 - 3.0 / 16.0 * t2887 * t2888 * t7787 * t1885 - 0.17149607247227894789e-2 * t17780;
    (t21394,)
}
