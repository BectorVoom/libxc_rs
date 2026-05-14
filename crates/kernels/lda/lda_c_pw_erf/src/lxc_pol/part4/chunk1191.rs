//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1191/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1191<F: Float>(t13146: F, t2562: F, t3727: F, t2566: F, t3709: F, t1325: F, t3787: F, t6945: F, t519: F, t6904: F, t13163: F, t13174: F, t13176: F, t13181: F, t13211: F, t13233: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t17585 = 32.0 / 135.0 * t13146;
    let t17587 = 8.0 / 45.0 * t3727 * t2562;
    let t17589 = 8.0 / 45.0 * t3709 * t2566;
    let t17591 = t1325 * t3787 * t6945;
    let t17592 = 32.0 / 45.0 * t17591;
    let t17594 = t519 * t3787 * t6904;
    let t17595 = 16.0 / 15.0 * t17594;
    let t17596 = 32.0 / 135.0 * t13163;
    let t17597 = 32.0 / 45.0 * t13174;
    let t17598 = 32.0 / 135.0 * t13176;
    let t17599 = 32.0 / 45.0 * t13181;
    let t17600 = 64.0 / 135.0 * t13211;
    let t17601 = 32.0 / 135.0 * t13233;
    (t17585, t17587, t17589, t17592, t17595, t17596, t17597, t17598, t17599, t17600, t17601)
}
