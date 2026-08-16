//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 771/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk771<F: Float>(t181: F, t944: F, t184: F, t786: F, t494: F, t509: F, t2095: F, t5021: F, t803: F, t933: F, t1268: F, t4615: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5064 = t944 * t181;
    let t5065 = t5064 * t184;
    let t5067 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5065 * t786;
    let t5068 = t494 * t509;
    let t5069 = t5068 * t184;
    let t5071 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t5069 * t786;
    let t5072 = t5021 * t2095;
    let t5076 = t933 * t803;
    let t5084 = t1268 * t4615;
    (t5064, t5065, t5067, t5068, t5069, t5071, t5072, t5076, t5084)
}
