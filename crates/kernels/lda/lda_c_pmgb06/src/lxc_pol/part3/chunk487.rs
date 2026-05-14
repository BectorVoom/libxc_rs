//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 487/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk487<F: Float>(t2064: F, t465: F, t137: F, t132: F, t479: F, t802: F, t405: F, t848: F, t1576: F, t1821: F, t1826: F, t525: F, t1831: F, t103: F, t1474: F, t1563: F, t1571: F, t1572: F, t1818: F, t1823: F, t1828: F, t1833: F, t2060: F) -> (F, F, F, F, F, F, F, F) {
    let t2065 = t465 * t2064;
    let t2066 = t137 * t2065;
    let t2068 = t132 * t2066 / 30.0;
    let t2070 = t802 * t479 / 30.0;
    let t2077 = t405 * t848;
    let t2079 = t1576 * t1821;
    let t2082 = t525 * t1826;
    let t2085 = t525 * t1831;
    let t2088 = t1563 + 0.011997222222222222 * t1474 + 0.011997222222222222 * t1818 - 0.023994444444444443 * t1823 + 0.07198333333333333 * t1828 + 0.07198333333333333 * t1833 + t1571 + 0.0044444444444444444 * t1572 + 0.0044444444444444444 * t2077 - 0.0022222222222222222 * t103 * t2079 + 0.013333333333333334 * t103 * t2082 + 0.013333333333333334 * t2060 * t2085;
    (t2065, t2066, t2068, t2070, t2079, t2082, t2085, t2088)
}
