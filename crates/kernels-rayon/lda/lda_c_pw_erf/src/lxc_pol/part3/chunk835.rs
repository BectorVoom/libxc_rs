//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 835/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk835(t4737: f64, t4740: f64, t4743: f64, t4745: f64, t4747: f64, t4752: f64, t4755: f64, t4757: f64, t4762: f64, t4765: f64, t4767: f64, t4772: f64, t4775: f64, t4779: f64, t4783: f64, t4787: f64, t4790: f64) -> f64 {
    let t5849 = t4737 + t4740 + t4743 + t4745 - t4747 - t4752 + t4755 - t4757 - t4762 + t4765 - t4767 - t4772 + t4775 + t4779 + t4783 - t4787 - t4790;
    t5849
}
