//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1000/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1000<F: Float>(t529: F, t6566: F, t108: F, t267: F, t821: F, t518: F, t6850: F, t1401: F, t6843: F, t2146: F, t4795: F, t6208: F) -> (F, F, F, F, F, F) {
    let t15595 = t529 * t6566;
    let t15607 = t821 * t108 * t267;
    let t15614 = t6850 * t518;
    let t15619 = t1401 * t6843;
    let t15672 = t2146 * t4795;
    let t15685 = t6208 * t518;
    (t15595, t15607, t15614, t15619, t15672, t15685)
}
