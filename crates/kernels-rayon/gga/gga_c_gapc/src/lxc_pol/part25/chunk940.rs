//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 940/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk940(t9468: f64, t9474: f64, t9478: f64, t9481: f64, t9483: f64, t9486: f64, t9488: f64, t9491: f64, t9494: f64, t9499: f64, t9502: f64, t9505: f64, t9509: f64) -> f64 {
    let t10842 = -0.2471588561924985691e-3_f64 * t9468 - 0.82386285397499523032e-5_f64 * t9474 + 0.6746961805555555556e-5_f64 * t9478 - 0.4637672555408563478e-4_f64 * t9481 - 0.21642471925239962898e-3_f64 * t9483 - 0.11254699860307667372e-6_f64 * t9486 + 0.55603792169291016668e-2_f64 * t9488 - 0.20240885416666666668e-4_f64 * t9491 - 0.20240885416666666668e-4_f64 * t9494 - 0.22202903123154399017e-4_f64 * t9499 + 0.11272120794395814009e-6_f64 * t9502 - 0.20041830772435757309e-6_f64 * t9505 + 0.55603792169291016668e-2_f64 * t9509;
    t10842
}
