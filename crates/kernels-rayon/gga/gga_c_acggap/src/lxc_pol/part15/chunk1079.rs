//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1079/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1079(t33574: f64, t8085: f64, t7987: f64, t9159: f64, t2226: f64, t33802: f64, t2131: f64, t2132: f64, t2385: f64, t847: f64, t2230: f64, t33429: f64) -> (f64, f64, f64, f64, f64) {
    let t38455 = t33574 * t8085;
    let t38458 = 0.34694512752820797848e1_f64 * t7987 * t9159;
    let t38471 = 0.17347256376410398924e1_f64 * t33802 * t2226;
    let t38474 = t2131 * t2132 * t2385 * t847;
    let t38481 = 0.17347256376410398924e1_f64 * t33429 * t2230;
    (t38455, t38458, t38471, t38474, t38481)
}
