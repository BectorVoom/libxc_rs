//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1283/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1283(t1459: f64, t34360: f64, t7547: f64, t7950: f64, t111371: f64, t1936: f64, t572: f64, t101705: f64, t7953: f64, t127453: f64, t129014: f64, t2040: f64, t28978: f64, t32377: f64, t573: f64, t5805: f64, t7324: f64, t7554: f64, t7944: f64, t8124: f64, t8725: f64, param_d: f64) -> f64 {
    let t129018 = 6.0_f64 * t1459 * t34360;
    let t129026 = 6.0_f64 * t7547 * t7950;
    let t129029 = 6.0_f64 * t572 * t111371 * t1936;
    let t129032 = 6.0_f64 * t572 * t101705 * t1936;
    let t129034 = 3.0_f64 * t7547 * t7953;
    let t129037 = t129014 * t573 * param_d + 6.0_f64 * t2040 * t28978 + 3.0_f64 * t5805 * t8725 + 6.0_f64 * t7324 * t8124 + 6.0_f64 * t7554 * t7944 + t127453 + t129018 + t129026 + t129029 + t129032 + t129034 + t32377;
    t129037
}
