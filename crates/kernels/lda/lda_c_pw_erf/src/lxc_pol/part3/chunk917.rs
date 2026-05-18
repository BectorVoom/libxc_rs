//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 917/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk917<F: Float>(t1244: F, t2061: F, t539: F, t331: F, t3478: F, t1250: F, t1275: F, t933: F, t1269: F, t3524: F, t3520: F, t325: F, t3504: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9762 = t1244 * t1244;
    let t9763 = F::new(1.0) / t9762;
    let t9772 = t2061 * t539;
    let t9774 = t331 * t3478;
    let t9777 = F::new(1.0) / t1244 / t1250;
    let t9782 = t933 * t1275;
    let t9784 = t933 * t1269;
    let t9786 = t331 * t3524;
    let t9788 = t331 * t3520;
    let t9806 = t325 * t3504;
    (t9763, t9772, t9774, t9777, t9782, t9784, t9786, t9788, t9806)
}
