//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 498/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk498<F: Float>(t2052: F, t2795: F, t2730: F, t55: F, t285: F, t1759: F, t2149: F, t54: F, t433: F, t2008: F, t2010: F, t2012: F, t2014: F, t2733: F, t2736: F) -> (F, F, F, F, F, F) {
    let t2796 = t2795 * t2052;
    let t2801 = t55 * t2730;
    let t2802 = t285 * t2801;
    let t2803 = t1759 * t2802;
    let t2805 = t54 * t2149;
    let t2806 = t285 * t2805;
    let t2807 = t433 * t2806;
    let t2811 = t2008 - F::new(0.22687409291590604) * t2803 + t2010 + F::new(0.22687409291590604) * t2807 + t2012 - F::new(0.04525483399593904) * t2733 + t2014 + F::new(0.04525483399593904) * t2736;
    (t2796, t2801, t2803, t2805, t2807, t2811)
}
