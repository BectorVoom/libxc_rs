//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 414/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk414<F: Float>(t479: F, t781: F, t473: F, t780: F, t483: F, t485: F, t163: F, t169: F, t299: F, t841: F, t1235: F, t1295: F) -> (F, F, F, F, F, F) {
    let t1908 = t781 * t479;
    let t1910 = t473 * t780;
    let t1912 = t1910 * t483 * t485;
    let t1919 = t169 * t299 * t841 * t163;
    let t1922 = F::new(4.0) / F::new(45.0) * t1235;
    let t1923 = F::new(4.0) / F::new(45.0) * t1295;
    (t1908, t1910, t1912, t1919, t1922, t1923)
}
