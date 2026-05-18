//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1039/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1039<F: Float>(t10793: F, t1193: F, t1354: F, t1166: F, t740: F, t2833: F, t2841: F, t10506: F, t1152: F, t421: F, t8085: F, t10512: F, t418: F) -> (F, F, F, F, F, F, F) {
    let t10795 = t10793 * t1193 * t1354;
    let t10797 = t740 * t1166;
    let t10799 = t10797 * t1193 * t1354;
    let t10802 = t2833 * t2841 * t1354;
    let t10806 = F::new(0.002972565416694299) * t1152 * t10506 * t1354;
    let t10808 = F::new(7.439549289525431e-06) * t8085 * t421;
    let t10811 = F::new(0.007901556131563792) * t418 * t10512 * t421;
    (t10795, t10797, t10799, t10802, t10806, t10808, t10811)
}
