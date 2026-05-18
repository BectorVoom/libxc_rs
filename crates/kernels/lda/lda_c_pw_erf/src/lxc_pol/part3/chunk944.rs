//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 944/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk944<F: Float>(t1: F, t2853: F, t3: F, t604: F, t1635: F, t4187: F, t1185: F, t119: F, t603: F, t1627: F, t1631: F, t4204: F) -> (F, F, F, F, F, F) {
    let t10678 = t2853 * t1 * t3 * t604;
    let t10680 = t4187 * t1635;
    let t10682 = t1185 * t1;
    let t10685 = F::new(2.8503734567901235e-05) * t10682 * t119 * t603;
    let t10686 = t4187 * t1627;
    let t10688 = t1631 * t4204;
    (t10678, t10680, t10682, t10685, t10686, t10688)
}
