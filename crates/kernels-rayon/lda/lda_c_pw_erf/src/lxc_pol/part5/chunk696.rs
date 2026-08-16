//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 696/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk696(t2131: f64, t6209: f64, t2120: f64, t2127: f64, t267: f64, t4468: f64, t4470: f64, t5793: f64, t5797: f64, t5799: f64, t5801: f64, t6161: f64, t6162: f64, t6185: f64, t6192: f64, t6197: f64, t6200: f64, t6202: f64, t6204: f64, t6207: f64) -> (f64, f64, f64, f64) {
    let t6211 = 8.0_f64 / 15.0_f64 * t6209 * t2131;
    let t6212 = t2120 * t2127;
    let t6213 = 16.0_f64 / 45.0_f64 * t6212;
    let t6214 = t5793 + t5797 + 2.0_f64 / 3.0_f64 * t5799 + 0.2431111111111111_f64 * t5801 - t6161 - 2.0_f64 / 45.0_f64 * t6162 - t6185 * t267 / 15.0_f64 + t4468 + t4470 - t6192 + t6197 + t6200 - t6202 + t6204 + t6207 + t6211 + t6213;
    (t6211, t6212, t6213, t6214)
}
