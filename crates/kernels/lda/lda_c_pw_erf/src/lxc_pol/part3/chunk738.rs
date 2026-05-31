//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 738/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk738<F: Float>(t225: F, t4713: F, t1931: F, t611: F, t1621: F, t838: F, t3956: F, t197: F, t521: F, t504: F, t2070: F, t185: F) -> (F, F, F, F, F, F, F, F) {
    let t4714 = t4713 * t225;
    let t4718 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1931 * t611;
    let t4719 = t838 * t1621;
    let t4721 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t3956;
    let t4722 = t521 * t197;
    let t4723 = t4722 * t504;
    let t4724 = t2070 * t4723;
    let t4726 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t185 * t4724;
    (t4714, t4718, t4719, t4721, t4722, t4723, t4724, t4726)
}
