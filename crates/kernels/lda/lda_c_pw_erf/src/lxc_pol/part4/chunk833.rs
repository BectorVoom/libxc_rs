//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 833/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk833<F: Float>(t577: F, t6205: F, t181: F, t784: F, t184: F, t2131: F, t2120: F, t2127: F, t267: F, t4468: F, t4470: F, t5793: F, t5797: F, t5799: F, t5801: F, t6161: F, t6162: F, t6185: F, t6192: F, t6197: F, t6200: F, t6202: F, t6204: F) -> (F, F, F, F, F, F) {
    let t6207 = 4.0 / 45.0 * t6205 * t577;
    let t6208 = t784 * t181;
    let t6209 = t6208 * t184;
    let t6211 = 8.0 / 15.0 * t6209 * t2131;
    let t6212 = t2120 * t2127;
    let t6213 = 16.0 / 45.0 * t6212;
    let t6214 = t5793 + t5797 + 2.0 / 3.0 * t5799 + 0.2431111111111111 * t5801 - t6161 - 2.0 / 45.0 * t6162 - t6185 * t267 / 15.0 + t4468 + t4470 - t6192 + t6197 + t6200 - t6202 + t6204 + t6207 + t6211 + t6213;
    (t6207, t6208, t6209, t6211, t6213, t6214)
}
