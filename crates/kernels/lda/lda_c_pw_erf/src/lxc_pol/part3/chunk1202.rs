//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1202/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1202<F: Float>(t3416: F, t5226: F, t1318: F, t1319: F, t2000: F, t2973: F, t1954: F, t4758: F, t954: F, t4753: F, t5231: F, t2967: F, t3589: F, t4776: F, t811: F) -> (F, F, F, F, F, F) {
    let t14166 = F::new(8.0) / F::new(15.0) * t3416 * t5226;
    let t14170 = F::new(8.0) / F::new(45.0) * t1318 * t1319 * t2000 * t2973;
    let t14174 = F::new(8.0) / F::new(15.0) * t1318 * t4758 * t1954 * t954;
    let t14176 = F::new(8.0) / F::new(9.0) * t4753 * t5231;
    let t14178 = F::new(8.0) / F::new(9.0) * t3416 * t5231;
    let t14183 = F::new(64.0) / F::new(81.0) * t1318 * t4776 * t811 * t3589 * t2967;
    (t14166, t14170, t14174, t14176, t14178, t14183)
}
