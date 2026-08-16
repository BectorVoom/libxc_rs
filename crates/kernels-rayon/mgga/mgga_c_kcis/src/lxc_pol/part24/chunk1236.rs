//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1236/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1236(t100130: f64, t100133: f64, t100136: f64, t100139: f64, t100142: f64, t100145: f64, t100148: f64, t100152: f64, t100157: f64, t100162: f64, t7788: f64, t96787: f64) -> f64 {
    let t100165 = -t96787 - 0.23168402777777777778e-3_f64 * t100130 + 0.61905925925925925925e-2_f64 * t100133 + 0.19345601851851851852e-2_f64 * t100136 + 0.7722800925925925926e-4_f64 * t100139 - 0.19345601851851851852e-2_f64 * t100142 + 0.12897067901234567901e-2_f64 * t100145 - 0.11607361111111111111e-1_f64 * t100148 + 0.69505208333333333334e-3_f64 * t7788 * t100152 - 0.69505208333333333334e-3_f64 * t7788 * t100157 + 0.208515625e-2_f64 * t7788 * t100162;
    t100165
}
