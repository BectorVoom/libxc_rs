//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 513/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk513<F: Float>(t1521: F, t1531: F, t1629: F, t1637: F, t1641: F, t2570: F, t2573: F, t2574: F, t2575: F, t2576: F, t2577: F, t2578: F, t2657: F, t2663: F, t2665: F, t1573: F, t1577: F, t1581: F, t1592: F, t1595: F, t1599: F, t1603: F, t163: F, t164: F, t169: F, t171: F, t1902: F, t1908: F, t1912: F, t1919: F, t2379: F) -> (F, F) {
    let t2666 = t2570 + t1629 + t1637 + t1641 - t1521 - t1531 - t2573 + t2574 - t2575 + t2576 + t2577 + t2578;
    let t2668 = t2657 + t2663 + t2665 + t2666;
    let t2673 = -t1573 + 0.06301081444628223 * t1902 + t1577 + t1581 - 0.031505407223141116 * t2379 * t164 - 0.06301081444628223 * t1908 - 0.003950778065781896 * t1912 - t1592 - t1595 - t1599 - t1603 + 0.017961351015381915 * t1919 - 0.005388405304614574 * t169 * t171 * t2668 * t163;
    (t2668, t2673)
}
