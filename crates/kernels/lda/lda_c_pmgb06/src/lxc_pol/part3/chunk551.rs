//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 551/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk551<F: Float>(t1594: F, t454: F, t2864: F, t439: F, t1382: F, t1447: F, t1600: F, t496: F, t1602: F, t507: F, t493: F, t1481: F, t529: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2865 = t454 * t1594;
    let t2866 = t2864 * t2865;
    let t2868 = F::new(2.0) / F::new(15.0) * t439 * t2866;
    let t2869 = t1447 * t1382;
    let t2870 = F::new(4.0) / F::new(45.0) * t2869;
    let t2871 = t496 * t1600;
    let t2872 = t507 * t1602;
    let t2873 = t2871 * t2872;
    let t2875 = F::new(2.0) / F::new(15.0) * t493 * t2873;
    let t2876 = t1481 * t529;
    (t2865, t2866, t2868, t2869, t2870, t2871, t2872, t2873, t2875, t2876)
}
