//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 836/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk836<F: Float>(t117: F, t123: F, t1650: F, t740: F, t2779: F, t1147: F, t398: F, t1193: F, t1354: F, t1166: F, t2833: F, t2841: F, t10506: F, t1152: F, t421: F, t8085: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10790 = t123 * t740 * t1650 * t117;
    let t10792 = 0.7561297733553868 * t2779;
    let t10793 = t1147 * t398;
    let t10795 = t10793 * t1193 * t1354;
    let t10797 = t740 * t1166;
    let t10799 = t10797 * t1193 * t1354;
    let t10802 = t2833 * t2841 * t1354;
    let t10806 = 0.002972565416694299 * t1152 * t10506 * t1354;
    let t10808 = 7.439549289525431e-06 * t8085 * t421;
    (t10790, t10792, t10793, t10795, t10797, t10799, t10802, t10806, t10808)
}
