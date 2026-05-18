//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 496/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk496<F: Float>(t2025: F, t208: F, t213: F, t871: F, t97: F, t588: F, t591: F, t872: F, t1424: F, t1448: F, t1505: F, t1518: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2026 = t2025 * t208;
    let t2027 = t2026 * t213;
    let t2029 = t871 * t97;
    let t2030 = t2029 * t588;
    let t2032 = t872 * t591;
    let t2034 = F::new(2.0) / F::new(135.0) * t1424;
    let t2035 = F::new(2.0) / F::new(135.0) * t1448;
    let t2036 = t1505 / F::new(45.0);
    let t2037 = t1518 / F::new(45.0);
    (t2026, t2027, t2029, t2030, t2032, t2034, t2035, t2036, t2037)
}
