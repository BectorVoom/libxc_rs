//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 518/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk518<F: Float>(t2093: F, t529: F, t166: F, t161: F, t486: F, t853: F, t1639: F, t851: F, t531: F, t831: F, t464: F, t813: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2094 = t2093 * t529;
    let t2095 = t166 * t2094;
    let t2097 = t161 * t2095 / F::new(30.0);
    let t2099 = t486 * t853 / F::new(30.0);
    let t2100 = t1639 * t851;
    let t2101 = t166 * t2100;
    let t2103 = t161 * t2101 / F::new(30.0);
    let t2105 = t831 * t531 / F::new(30.0);
    let t2106 = t813 * t464;
    (t2094, t2095, t2097, t2099, t2100, t2101, t2103, t2105, t2106)
}
