//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 747/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk747<F: Float>(t1944: F, t4794: F, t571: F, t4743: F, t4745: F, t4747: F, t4752: F, t4755: F, t4757: F, t4762: F, t4765: F, t4767: F, t4772: F, t4775: F, t4779: F, t4783: F, t4787: F, t4790: F, t4793: F) -> (F, F, F) {
    let t4795 = t4794 * t1944;
    let t4797 = F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t571 * t4795;
    let t4798 = t4743 + t4745 - t4747 - t4752 + t4755 - t4757 - t4762 + t4765 - t4767 - t4772 + t4775 + t4779 + t4783 - t4787 - t4790 - t4793 + t4797;
    (t4795, t4797, t4798)
}
