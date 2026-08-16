//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1157/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1157<F: Float>(t10335: F, t17771: F, t20888: F, t20890: F, t20892: F, t20894: F, t20895: F, t20896: F, t20897: F, t20898: F, t20901: F, t1385: F, t439: F, t6217: F, t822: F) -> (F, F, F, F) {
    let t20902 = F::cast_from(8.0_f64) / F::cast_from(1215.0_f64) * t10335;
    let t20903 = t17771 / F::cast_from(15.0_f64);
    let t20904 = -t20888 - t20890 - t20892 + t20894 + t20895 + t20896 - t20897 + t20898 + t20901 + t20902 + t20903;
    let t20914 = t439 * t1385 * t6217 * t822 / F::cast_from(15.0_f64);
    (t20902, t20903, t20904, t20914)
}
