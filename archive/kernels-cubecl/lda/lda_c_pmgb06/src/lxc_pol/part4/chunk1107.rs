//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1107/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1107<F: Float>(t161: F, t4802: F, t489: F, t132: F, t1547: F, t2042: F, t1963: F, t3213: F, t1423: F, t4772: F, t1710: F, t801: F) -> (F, F, F, F, F) {
    let t13686 = t161 * t489 * t4802;
    let t13706 = t132 * t1547 * t2042;
    let t13708 = t3213 * t1963;
    let t13710 = t1423 * t4772;
    let t13712 = t801 * t1710;
    (t13686, t13706, t13708, t13710, t13712)
}
