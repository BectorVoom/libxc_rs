//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 745/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk745<F: Float>(t4666: F, t4776: F, t571: F, t1287: F, t816: F, t1319: F, t1318: F, t1954: F, t549: F, t4758: F, t2010: F, t3863: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4777 = t4776 * t4666;
    let t4779 = F::new(32.0) / F::new(81.0) * t571 * t4777;
    let t4780 = t816 * t1287;
    let t4781 = t1319 * t4780;
    let t4783 = F::new(8.0) / F::new(45.0) * t1318 * t4781;
    let t4784 = t1954 * t549;
    let t4785 = t4758 * t4784;
    let t4787 = F::new(32.0) / F::new(45.0) * t1318 * t4785;
    let t4788 = t3863 * t2010;
    (t4777, t4779, t4780, t4781, t4783, t4784, t4785, t4787, t4788)
}
