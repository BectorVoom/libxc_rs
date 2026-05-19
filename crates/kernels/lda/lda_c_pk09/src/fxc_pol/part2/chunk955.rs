//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 955/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk955<F: Float>(t10048: F, t10050: F, t10052: F, t10060: F, t10062: F, t1451: F, t2607: F, t5632: F, t5714: F, t9885: F, t9887: F, t9890: F, t9892: F) -> F {
    let t10067 = -t10048 / F::new(18.0) - t10050 / F::new(18.0) - t10052 / F::new(18.0) - F::cast_from(0.14975624337724558_f64) * t9885 - F::cast_from(0.14975624337724558_f64) * t9887 + F::cast_from(0.037002892246025966_f64) * t9890 + F::cast_from(0.037002892246025966_f64) * t9892 - t10060 / F::new(18.0) - t10062 * t1451 / F::new(6.0) - t2607 * t5632 / F::new(6.0) + t5714;
    t10067
}
