//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 623/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk623<F: Float>(t3638: F, t3614: F, t557: F, t11: F, t3619: F, t1357: F, t325: F, t1349: F, t3605: F, t3610: F, t3625: F, t3627: F, t3629: F, t3631: F, t3635: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3639 = F::cast_from(0.11197407407407407_f64) * t3638;
    let t3640 = t557 * t3614;
    let t3641 = t11 * t3640;
    let t3643 = t557 * t3619;
    let t3644 = t11 * t3643;
    let t3646 = t325 * t1357;
    let t3648 = t1349 * t3605;
    let t3649 = t11 * t3648;
    let t3651 = t1349 * t3610;
    let t3652 = t11 * t3651;
    let t3654 = -F::cast_from(0.035991666666666665_f64) * t3625 - F::cast_from(0.047988888888888886_f64) * t3627 + F::cast_from(0.035991666666666665_f64) * t3629 + F::cast_from(0.023994444444444443_f64) * t3631 - F::cast_from(0.03999074074074074_f64) * t3635 - t3639 - F::cast_from(0.21595_f64) * t3641 + F::cast_from(0.21595_f64) * t3644 - F::cast_from(0.07198333333333333_f64) * t3646 + F::cast_from(0.14396666666666666_f64) * t3649 - F::cast_from(0.07198333333333333_f64) * t3652;
    (t3639, t3640, t3641, t3643, t3644, t3646, t3648, t3649, t3651, t3652, t3654)
}
