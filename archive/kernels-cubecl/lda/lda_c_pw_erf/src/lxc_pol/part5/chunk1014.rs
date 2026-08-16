//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1014/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1014<F: Float>(t2146: F, t4901: F, t4763: F, t4933: F, t611: F, t7280: F, t1472: F, t6685: F, t1518: F, t211: F, t2527: F, t2526: F, t3975: F) -> (F, F, F, F, F, F) {
    let t16516 = t2146 * t4901;
    let t16520 = t4763 * t4933;
    let t16529 = t7280 * t611;
    let t16537 = t1472 * t6685;
    let t16600 = t211 * t1518 * t2527;
    let t16602 = t3975 * t2526;
    (t16516, t16520, t16529, t16537, t16600, t16602)
}
