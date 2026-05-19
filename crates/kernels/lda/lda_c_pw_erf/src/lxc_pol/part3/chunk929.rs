//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 929/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk929<F: Float>(t1361: F, t925: F, t9812: F, t155: F, t188: F, t1325: F, t1442: F, t2176: F, t524: F, t519: F, t1612: F, t610: F) -> (F, F, F, F, F, F, F) {
    let t10115 = t925 * t1361;
    let t10145 = F::cast_from(0.01959135802469136_f64) * t9812;
    let t10162 = t155 * t188;
    let t10164 = t1325 * t10162 * t1442;
    let t10166 = t2176 * t524;
    let t10167 = t519 * t10166;
    let t10169 = t1612 * t610;
    (t10115, t10145, t10162, t10164, t10166, t10167, t10169)
}
