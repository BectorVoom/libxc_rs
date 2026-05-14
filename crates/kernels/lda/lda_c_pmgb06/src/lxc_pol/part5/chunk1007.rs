//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1007/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1007<F: Float>(t20830: F, t2015: F, t2592: F, t17651: F, t6621: F, t802: F, t20810: F, t20813: F, t20816: F, t20818: F, t20820: F, t20822: F, t20824: F, t20828: F, t161: F, t16595: F, t166: F, t851: F) -> (F, F, F, F, F, F) {
    let t20831 = t20830 / 15.0;
    let t20832 = t2592 * t2015;
    let t20833 = t20832 / 15.0;
    let t20834 = 2.0 / 15.0 * t17651;
    let t20835 = t802 * t6621;
    let t20836 = t20835 / 15.0;
    let t20837 = t20810 - t20813 - t20816 + t20818 + t20820 - t20822 - t20824 - t20828 - t20831 - t20833 + t20834 - t20836;
    let t20843 = t161 * t166 * t16595 * t851 / 10.0;
    (t20831, t20833, t20834, t20836, t20837, t20843)
}
