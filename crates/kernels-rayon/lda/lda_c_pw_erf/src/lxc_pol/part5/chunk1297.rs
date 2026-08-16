//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1297/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1297(t164: f64, t20658: f64, t10778: f64, t10783: f64, t10787: f64, t10788: f64, t10791: f64, t10793: f64, t10800: f64, t10802: f64, t10808: f64, t10812: f64, t10816: f64, t11667: f64, t18755: f64, t18761: f64) -> f64 {
    let t23173 = t20658 * t164;
    let t23176 = -0.0014862827083471494_f64 * t10778 - t10783 - t10787 - 0.025899545097903542_f64 * t10788 - t10791 - t10793 + t10800 + t10802 + 0.01975389032890948_f64 * t10808 + 0.0034679929861433484_f64 * t10812 + t10816 + 0.01975389032890948_f64 * t18755 - t11667 + 0.031505407223141116_f64 * t23173 + 0.02694202652307287_f64 * t18761;
    t23176
}
