//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 644/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk644<F: Float>(t2062: F, t5021: F, t830: F, t933: F, t1386: F, t2120: F, t1234: F, t795: F, t1294: F, t822: F, t2095: F, t803: F) -> (F, F, F, F, F, F, F) {
    let t5022 = t5021 * t2062;
    let t5024 = t933 * t830;
    let t5039 = F::new(16.0) / F::new(45.0) * t2120 * t1386;
    let t5055 = F::new(8.0) / F::new(45.0) * t795 * t1234;
    let t5057 = F::new(8.0) / F::new(45.0) * t822 * t1294;
    let t5072 = t5021 * t2095;
    let t5076 = t933 * t803;
    (t5022, t5024, t5039, t5055, t5057, t5072, t5076)
}
