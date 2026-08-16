//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1250/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1250(t10921: f64, t10942: f64, t10988: f64, t11170: f64, t39167: f64, t39168: f64, t39169: f64, t39170: f64, t39171: f64, t39172: f64, t39173: f64, t39174: f64, t39175: f64, t39176: f64, t39177: f64, t40724: f64, t40726: f64, t40728: f64, t40729: f64, t40735: f64, t41090: f64, t41092: f64, t41098: f64, t8: f64) -> f64 {
    let t41103 = -t39167 - t39168 - t10921 + t10942 + t39169 + t39170 + t39171 - t39172 + t11170 + t39173 - t39174 + t10988 - t39175 + t39176 + t39177 + t8 * (t40724 + t40726 + t40728 + t40729 + t40735 + t41090 + t41092 + t41098);
    t41103
}
