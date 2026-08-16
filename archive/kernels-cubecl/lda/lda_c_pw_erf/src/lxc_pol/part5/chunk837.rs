//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 837/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk837<F: Float>(t2437: F, t784: F, t1326: F, t1325: F, t806: F, t1313: F, t519: F, t7651: F, t1991: F, t7643: F, t2429: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7687 = t2437 * t784;
    let t7688 = t1326 * t7687;
    let t7690 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1325 * t7688;
    let t7691 = t2437 * t806;
    let t7692 = t1313 * t7691;
    let t7694 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t519 * t7692;
    let t7695 = t1326 * t7651;
    let t7697 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t519 * t7695;
    let t7698 = t1991 * t7643;
    let t7700 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t519 * t7698;
    let t7701 = t2429 * t784;
    let t7702 = t1991 * t7701;
    (t7687, t7688, t7690, t7691, t7692, t7694, t7695, t7697, t7698, t7700, t7701, t7702)
}
