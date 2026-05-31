//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 644/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk644<F: Float>(t504: F, t944: F, t348: F, t1326: F, t1325: F, t1310: F, t1472: F, t1360: F, t593: F, t1308: F, t571: F, t1381: F, t559: F) -> (F, F, F, F, F, F, F, F) {
    let t3817 = t944 * t504;
    let t3818 = t3817 * t348;
    let t3819 = t1326 * t3818;
    let t3821 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1325 * t3819;
    let t3823 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1472 * t1310;
    let t3824 = t1360 * t593;
    let t3825 = t1308 * t3824;
    let t3827 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t571 * t3825;
    let t3828 = t559 * t1381;
    (t3818, t3819, t3821, t3823, t3824, t3825, t3827, t3828)
}
