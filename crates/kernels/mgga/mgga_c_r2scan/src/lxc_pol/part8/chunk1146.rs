//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1146/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1146<F: Float>(t5357: F, t695: F, t1719: F, t1822: F, t1923: F, t1945: F, t1957: F, t1966: F, t2005: F, t206: F, t21375: F, t21379: F, t21387: F, t21404: F, t21430: F, t224: F, t390: F, t5261: F, t5272: F, t5530: F, t5564: F, t5589: F, t5628: F, t5695: F, t5697: F, t5740: F, t5748: F, t5755: F, t5782: F, t681: F, t721: F) -> (F, F, F) {
    let t21594 = t5357 * t695;
    let t21601 = t1822 * t1719;
    let t21613 = -0.11407595979765752406e3 * t390 * t5261 + 0.11978736182162784439e7 * t5695 * t206 * t5697 * t1923 * t1966 + t21375 + t21379 - t21387 - 0.99313944805004928578e5 * t5748 * t5740 * t5589 - 0.31168546390226634765e3 * t1945 * t721 * t21430 + 0.14035736694323150897e2 * t5564 * t21594 - 0.23158050478117250496e4 * t1957 * t681 * t5782 - 0.73828935779158127934e5 * t5530 * t224 * t21601 + t21404 - 0.11579025239058625248e4 * t1957 * t5755 * t1923 - 0.42514644538609193172e3 * t390 * t2005 * t681 * t5628 - 0.33776465721256572866e4 * t390 * t5272;
    (t21594, t21601, t21613)
}
