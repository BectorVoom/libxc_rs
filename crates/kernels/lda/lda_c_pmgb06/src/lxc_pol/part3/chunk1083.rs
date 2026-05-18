//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1083/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1083<F: Float>(t12864: F, t1915: F, t493: F, t1912: F, t3223: F, t1916: F, t12839: F, t12844: F, t12846: F, t12849: F, t12852: F, t12855: F, t12857: F, t12859: F, t12863: F) -> (F, F, F, F) {
    let t12867 = F::new(8.0) / F::new(15.0) * t493 * t1915 * t12864;
    let t12868 = t3223 * t1912;
    let t12869 = F::new(2.0) / F::new(135.0) * t12868;
    let t12870 = t3223 * t1916;
    let t12871 = F::new(4.0) / F::new(135.0) * t12870;
    let t12872 = t12839 + t12844 - t12846 - t12849 - t12852 + t12855 - t12857 - t12859 - t12863 - t12867 + t12869 + t12871;
    (t12867, t12869, t12871, t12872)
}
