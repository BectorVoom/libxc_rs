//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3378/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3378(t11354: f64, t2881: f64, t6120: f64, t41382: f64, t6113: f64, t11358: f64, t42731: f64, t52011: f64, t60927: f64, t63468: f64, t916: f64, t41330: f64, t41332: f64, t63474: f64, t63476: f64, t63478: f64, t63480: f64, t63482: f64, t63485: f64, t63488: f64, t63491: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t63494 = t11354 * t6120 * t2881;
    let t63497 = t41382 * t6113 * t2881;
    let t63500 = t11358 * t6120 * t2881;
    let t63503 = t52011 * t42731 * t60927;
    let t63505 = t916 * t63468;
    let t63509 = -0.1898925e1_f64 * t63474 - 0.9494625e0_f64 * t63476 - 0.76790625e-1_f64 * t63478 + 0.3071625e0_f64 * t63480 + 0.15358125e0_f64 * t63482 - 0.1898925e1_f64 * t63485 + 0.3071625e0_f64 * t63488 - 0.3560484375e1_f64 * t63491 + 0.142419375e1_f64 * t63494 + 0.1151859375e0_f64 * t63497 - 0.76790625e-1_f64 * t63500 + 0.65725333333333333333e0_f64 * t63503 + 0.1898925e1_f64 * t63505 - 0.13287407407407407408e0_f64 * t41330 - 0.88582716049382716053e-1_f64 * t41332;
    (t63494, t63497, t63500, t63503, t63505, t63509)
}
