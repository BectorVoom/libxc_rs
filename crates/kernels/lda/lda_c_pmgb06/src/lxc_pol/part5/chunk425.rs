//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 425/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk425<F: Float>(t1545: F, t432: F, t824: F, t1395: F, t822: F, t137: F, t132: F, t405: F, t819: F, t1619: F, t1859: F, t1864: F, t473: F, t1669: F, t99: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2039 = t1545 / 45.0;
    let t2041 = t432 * t824 / 30.0;
    let t2042 = t1395 * t822;
    let t2043 = t137 * t2042;
    let t2045 = t132 * t2043 / 30.0;
    let t2052 = t405 * t819;
    let t2054 = t1619 * t1859;
    let t2057 = t473 * t1864;
    let t2060 = t99 * t1669;
    (t2039, t2041, t2042, t2043, t2045, t2052, t2054, t2057, t2060)
}
