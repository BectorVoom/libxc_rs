//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1069/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1069(t7732: f64, t8749: f64, t2007: f64, t2127: f64, t33575: f64, t33578: f64, t33580: f64, t33583: f64, t33587: f64, t33589: f64, t33592: f64, t33595: f64, t33599: f64, t34377: f64, t7883: f64, t8152: f64) -> f64 {
    let t34379 = t7732 * t8749;
    let t34381 = -t2007 * t8152 - t2127 * t7883 - 2.0_f64 * t33575 - t33578 - t33580 - t33583 - 2.0_f64 * t33587 - 2.0_f64 * t33589 - 2.0_f64 * t33592 - t33595 - t33599 - 2.0_f64 * t34377 - 2.0_f64 * t34379;
    t34381
}
