//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2829/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2829(t14791: f64, t2745: f64, t40409: f64, t50370: f64, t50372: f64, t50375: f64, t50377: f64, t50381: f64, t50383: f64, t50385: f64, t50387: f64, t50390: f64, t6035: f64, t61572: f64, t61574: f64, t61576: f64, t61582: f64, t61612: f64, t61616: f64, t61749: f64, t76302: f64, t837: f64) -> f64 {
    let t76458 = 0.4065600224742826258e-3_f64 * t61572 + 0.30011812682648815881e-2_f64 * t61574 + 0.4065600224742826258e-3_f64 * t61576 - 0.85748036236139473944e-4_f64 * t61582 - 0.20082057720118594944e-6_f64 * t40409 + 0.45351183609335988442e0_f64 * t50370 + 0.21675198048579700358e-2_f64 * t50372 - t50375 - 0.24098469264142313933e-5_f64 * t50377 + 0.33884236873090992593e-6_f64 * t50381 - 0.68026775414003982663e-1_f64 * t50383 - 0.15415400852149882895e-1_f64 * t50385 + 0.45732285992607719436e-2_f64 * t50387 + t50390 + 0.25724410870841842183e-2_f64 * t2745 * t14791 * t61749 * t6035 + 0.25724410870841842183e-2_f64 * t2745 * t14791 * t76302 * t837 - 0.17149607247227894789e-3_f64 * t61612 - 0.17149607247227894789e-3_f64 * t61616;
    t76458
}
