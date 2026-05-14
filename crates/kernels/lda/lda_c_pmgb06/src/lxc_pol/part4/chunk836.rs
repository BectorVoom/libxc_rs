//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 836/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk836<F: Float>(t1395: F, t2648: F, t137: F, t132: F, t2043: F, t802: F, t2066: F, t2650: F, t432: F, t2625: F, t486: F, t1639: F, t2623: F, t166: F, t161: F, t4815: F, t822: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6583 = t1395 * t2648;
    let t6584 = t137 * t6583;
    let t6586 = t132 * t6584 / 30.0;
    let t6588 = t802 * t2043 / 15.0;
    let t6590 = t802 * t2066 / 15.0;
    let t6592 = t432 * t2650 / 30.0;
    let t6594 = t486 * t2625 / 30.0;
    let t6595 = t1639 * t2623;
    let t6596 = t166 * t6595;
    let t6598 = t161 * t6596 / 30.0;
    let t6599 = t4815 * t822;
    (t6583, t6584, t6586, t6588, t6590, t6592, t6594, t6595, t6596, t6598, t6599)
}
