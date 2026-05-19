//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1251/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1251<F: Float>(t10350: F, t10353: F, t10356: F, t10358: F, t10362: F, t18329: F, t18331: F, t20902: F, t20903: F, t20914: F, t20919: F, t20920: F, t20922: F, t20925: F, t20929: F, t20931: F, t20934: F, t20937: F, t20940: F, t20943: F, t20946: F, t20950: F, t20953: F) -> (F, F) {
    let t22036 = t20902 + t20903 - F::new(2.0) / F::new(9.0) * t10350 - F::cast_from(0.013506172839506173_f64) * t10353 - t10356 - t10358 + t10362 - F::new(2.0) / F::new(15.0) * t18329 + F::new(2.0) / F::new(45.0) * t18331 - t20914 - t20919;
    let t22037 = t20920 - t20922 - t20925 - t20929 - t20931 - t20934 - t20937 + t20940 - t20943 - t20946 - t20950 - t20953;
    (t22036, t22037)
}
