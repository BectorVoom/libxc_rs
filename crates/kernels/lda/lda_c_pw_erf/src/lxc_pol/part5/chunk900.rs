//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 900/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk900<F: Float>(t12118: F, t6713: F, t6717: F, t6720: F, t10011: F, t6759: F, t6763: F, t6767: F, t2497: F, t4489: F, t1325: F, t3859: F, t6264: F, t2171: F, t5421: F, t3863: F, t571: F, t6408: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16624 = t12118 * t6713;
    let t16626 = t12118 * t6717;
    let t16633 = t12118 * t6720;
    let t16648 = t10011 * t6759;
    let t16650 = t10011 * t6763;
    let t16652 = t10011 * t6767;
    let t16657 = t4489 * t2497;
    let t16702 = t1325 * t3859 * t6264;
    let t16709 = t2171 * t5421;
    let t16762 = t571 * t3863 * t6408;
    (t16624, t16626, t16633, t16648, t16650, t16652, t16657, t16702, t16709, t16762)
}
