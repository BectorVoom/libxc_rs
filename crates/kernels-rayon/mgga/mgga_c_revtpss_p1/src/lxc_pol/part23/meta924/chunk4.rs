//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2992/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2992(t41361: f64, t42013: f64, t51978: f64, t52946: f64, t63276: f64, t63278: f64, t77499: f64, t77503: f64, t77505: f64, t77507: f64, t77509: f64, t77515: f64, t77518: f64, t77521: f64, t77527: f64, t77531: f64, t77535: f64, t77539: f64, t77543: f64, t77547: f64) -> f64 {
    let t79366 = 0.30864197530864197531e-2_f64 * t77499 - 0.83333333333333333333e-2_f64 * t77503 + 0.27777777777777777778e-2_f64 * t77505 - 0.11111111111111111111e-1_f64 * t77507 + 0.16666666666666666667e-1_f64 * t77509 - 0.16666666666666666667e-1_f64 * t63276 + 0.55555555555555555556e-2_f64 * t63278 + t42013 + 0.99999999999999999998e-1_f64 * t77515 - 0.27777777777777777777e-1_f64 * t77518 - 0.15e0_f64 * t77521 - t52946 + 0.25925925925925925926e-1_f64 * t51978 + 0.86419753086419753087e-2_f64 * t41361 - 0.16666666666666666666e-1_f64 * t77527 - 0.16666666666666666666e-1_f64 * t77531 + 0.2e0_f64 * t77535 - 0.15e0_f64 * t77539 + 0.50000000000000000001e-1_f64 * t77543 + 0.50000000000000000001e-1_f64 * t77547;
    t79366
}
