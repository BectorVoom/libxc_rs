//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 749/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk749<F: Float>(t133: F, t5506: F, t5521: F, t3280: F, t3284: F, t3322: F, t3348: F, t3361: F, t5550: F, t5570: F, t5577: F, t5588: F, t5591: F, t5609: F, t5658: F, t138: F, t1706: F, t1711: F, t1712: F, t1724: F, t1861: F, t1864: F, t1878: F, t3329: F, t3332: F, t3339: F, t444: F, t450: F, t5616: F, t5618: F, t5621: F, t5630: F, t5633: F, t5636: F, t774: F) -> (F, F) {
    let t5660 = t133 * t5506;
    let t5663 = 1.1495033333333333 * t133 * t5521;
    let t5666 = -1.724255 * t3361 + t3280 - t3284 - t5570 - t3348 - t5577 + t5588 + t5591 - 0.7663355555555555 * t5660 + t5663 - 1.724255 * t133 * t5550 - t5609 - t3322;
    let t5667 = t5658 + t5666;
    let t5669 = t5616 * t138 - 2.0 * t1706 * t1878 + 4.0 * t1711 * t5633 + 2.0 * t1711 * t5636 + 2.0 * t5621 * t1712 - t1861 * t1724 + 4.0 * t3332 * t1864 - t3329 * t774 - 6.0 * t3339 * t5630 - t444 * t5667 - 2.0 * t5618 * t450;
    (t5667, t5669)
}
