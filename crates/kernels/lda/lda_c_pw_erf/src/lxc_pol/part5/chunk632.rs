//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 632/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk632<F: Float>(t2849: F, t1931: F, t611: F, t1621: F, t838: F, t197: F, t521: F, t1518: F, t807: F, t185: F, t230: F, t610: F) -> (F, F, F, F, F, F, F, F) {
    let t4712 = F::new(12.0) * t2849;
    let t4718 = F::new(8.0) / F::new(3.0) * t1931 * t611;
    let t4719 = t838 * t1621;
    let t4722 = t521 * t197;
    let t4729 = t1518 * t807;
    let t4730 = t185 * t4729;
    let t4733 = F::new(8.0) / F::new(3.0) * t1931 * t230;
    let t4734 = t838 * t610;
    (t4712, t4718, t4719, t4722, t4729, t4730, t4733, t4734)
}
