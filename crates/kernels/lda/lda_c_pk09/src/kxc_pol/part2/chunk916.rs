//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 916/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk916<F: Float>(t1777: F, t2758: F, t11248: F, t1823: F, t1672: F, t2877: F, t1859: F, t1468: F, t2777: F, t1782: F, t1838: F, t1841: F, t2778: F, t1971: F, t2902: F, t11101: F, t1831: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11299 = t1777 * t2758;
    let t11302 = t1823 * t11248;
    let t11304 = t2877 * t1672;
    let t11306 = t1859 * t11248;
    let t11310 = t2777 * t1468;
    let t11311 = t11310 * t1782;
    let t11312 = t11311 * t1838;
    let t11314 = t2778 * t1841;
    let t11317 = t2902 * t1971;
    let t11322 = t1831 * t11101;
    (t11299, t11302, t11304, t11306, t11311, t11312, t11314, t11317, t11322)
}
