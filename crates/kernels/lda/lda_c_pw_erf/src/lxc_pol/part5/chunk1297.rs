//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1297/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1297<F: Float>(t164: F, t20658: F, t10778: F, t10783: F, t10787: F, t10788: F, t10791: F, t10793: F, t10800: F, t10802: F, t10808: F, t10812: F, t10816: F, t11667: F, t18755: F, t18761: F) -> F {
    let t23173 = t20658 * t164;
    let t23176 = -F::cast_from(0.0014862827083471494_f64) * t10778 - t10783 - t10787 - F::cast_from(0.025899545097903542_f64) * t10788 - t10791 - t10793 + t10800 + t10802 + F::cast_from(0.01975389032890948_f64) * t10808 + F::cast_from(0.0034679929861433484_f64) * t10812 + t10816 + F::cast_from(0.01975389032890948_f64) * t18755 - t11667 + F::cast_from(0.031505407223141116_f64) * t23173 + F::cast_from(0.02694202652307287_f64) * t18761;
    t23176
}
