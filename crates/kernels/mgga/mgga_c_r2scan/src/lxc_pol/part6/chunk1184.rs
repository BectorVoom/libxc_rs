//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1184/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1184<F: Float>(t207: F, t5549: F, t5791: F, t644: F, t650: F, t1399: F, t1691: F, t1719: F, t1743: F, t1821: F, t1827: F, t1842: F, t1931: F, t1981: F, t1982: F, t2006: F, t21115: F, t21409: F, t390: F, t5270: F, t5280: F, t5429: F, t5503: F, t5507: F, t5530: F, t5537: F, t5542: F, t5593: F, t5658: F, t5787: F, t681: F, t695: F, t705: F, t718: F, t720: F) -> (F, F, F) {
    let t21621 = t207 * t5549;
    let t21649 = 72.0 * t650 * t644 * t5791;
    let t21654 = -0.19263893255070628432e1 * t390 * t5280 - 0.17349730080482783747e0 * t1399 * t1827 + 0.25685191006760837908e1 * t1399 * t1842 + 0.13698666666666666667e0 * t390 * t1931 * t21621 + 0.41095999999999999999e0 * t390 * t5503 * t5507 - t21409 - 0.49219290519438751956e5 * t5530 * t5593 * t5270 + 0.10389515463408878255e3 * t718 * t5542 * t1719 + 0.61524113149298439946e4 * t1981 * t1743 * t1821 * t1691 + 0.12304822629859687989e6 * t5537 * t1982 * t21115 - 0.46785788981077169656e1 * t705 * t5658 * t695 + 0.24828486201251232144e5 * t2006 * t681 * t5787 - t21649 + 0.69263436422725855036e2 * t718 * t5429 * t720 * t695;
    (t21621, t21649, t21654)
}
