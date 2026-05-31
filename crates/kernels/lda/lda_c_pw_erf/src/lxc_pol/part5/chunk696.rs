//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 696/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk696<F: Float>(t2131: F, t6209: F, t2120: F, t2127: F, t267: F, t4468: F, t4470: F, t5793: F, t5797: F, t5799: F, t5801: F, t6161: F, t6162: F, t6185: F, t6192: F, t6197: F, t6200: F, t6202: F, t6204: F, t6207: F) -> (F, F, F, F) {
    let t6211 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t6209 * t2131;
    let t6212 = t2120 * t2127;
    let t6213 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t6212;
    let t6214 = t5793 + t5797 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5799 + F::cast_from(0.2431111111111111_f64) * t5801 - t6161 - F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t6162 - t6185 * t267 / F::cast_from(15.0_f64) + t4468 + t4470 - t6192 + t6197 + t6200 - t6202 + t6204 + t6207 + t6211 + t6213;
    (t6211, t6212, t6213, t6214)
}
