//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3613/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3613(t20361: f64, t3399: f64, t20365: f64, t16926: f64, t5087: f64, t1134: f64, t20337: f64, t3407: f64, t20370: f64, t20356: f64, t58145: f64, t58147: f64, t68470: f64, t68473: f64, t68476: f64, t68479: f64, t68481: f64, t68484: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t68486 = t20361 * t3399;
    let t68488 = t20365 * t3399;
    let t68490 = t5087 * t16926;
    let t68493 = t3407 * t20337 * t1134;
    let t68495 = t20370 * t3399;
    let t68497 = t20356 * t3399;
    let t68501 = -0.485484375e1_f64 * t68470 + 0.19419375e1_f64 * t68473 + 0.6189328125e-1_f64 * t68476 - 0.412621875e-1_f64 * t68479 - 0.258925e1_f64 * t68481 - 0.258925e1_f64 * t68484 - 0.1294625e1_f64 * t68486 - 0.412621875e-1_f64 * t68488 + 0.16504875e0_f64 * t68490 + 0.16504875e0_f64 * t68493 + 0.82524375e-1_f64 * t68495 + 0.19419375e1_f64 * t68497 + 0.36793333333333333334e0_f64 * t58145 - 0.11038e0_f64 * t58147;
    (t68486, t68488, t68490, t68493, t68495, t68497, t68501)
}
