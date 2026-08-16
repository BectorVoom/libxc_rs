//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2873/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2873(t15389: f64, t2918: f64, t2924: f64, t11387: f64, t4631: f64, t11385: f64, t2875: f64, t51840: f64, t51844: f64, t51846: f64, t52141: f64, t52146: f64, t52150: f64, t52153: f64, t52156: f64, t52159: f64) -> (f64, f64, f64) {
    let t52162 = 0.48245938496077605201e2_f64 * t2924 * t15389 * t2918;
    let t52163 = t4631 * t11387;
    let t52166 = 0.1551780387578202009e4_f64 * t11385 * t52163 * t2875;
    let t52167 = t51840 - t51844 + t51846 - t52141 - t52146 + t52150 - t52153 - t52156 - t52159 + t52162 + t52166;
    (t52162, t52166, t52167)
}
