//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 811/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk811<F: Float>(t3210: F, t8920: F, t119: F, t155: F, t3251: F, t1657: F, t1: F, t2824: F, t125: F, t2715: F, t3310: F, t3319: F, t8138: F, t1125: F, t427: F, t426: F) -> (F, F, F, F, F, F, F, F) {
    let t8921 = t3210 * t8920;
    let t8924 = t119 * t155 * t3251;
    let t8925 = t1657 * t8924;
    let t8930 = t2824 * t1 * t119;
    let t8932 = 0.16322666666666666 * t125 * t2715 * t3310 * t8930;
    let t8936 = 1.6239027777777777 * param_hyb_omega_0 * t8138 * t3319 * t8930;
    let t8939 = t1125 * t427;
    let t8940 = t426 * t8939;
    (t8921, t8924, t8925, t8930, t8932, t8936, t8939, t8940)
}
