//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 850/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk850<F: Float>(t1325: F, t3407: F, t3859: F, t219: F, t3762: F, t1318: F, t1321: F, t156: F, t4195: F, t602: F, t1: F, t2853: F, t3: F, t604: F, t1635: F, t4187: F) -> (F, F, F, F, F, F) {
    let t10643 = t1325 * t3859 * t3407;
    let t10654 = t3762 * t219;
    let t10656 = t1318 * t10654 * t1321;
    let t10675 = 0.4328416544945937 * t602 * t156 * t4195;
    let t10678 = t2853 * t1 * t3 * t604;
    let t10680 = t4187 * t1635;
    (t10643, t10654, t10656, t10675, t10678, t10680)
}
