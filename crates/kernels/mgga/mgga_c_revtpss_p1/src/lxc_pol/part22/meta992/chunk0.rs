//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3378/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3378<F: Float>(t11354: F, t2881: F, t6120: F, t41382: F, t6113: F, t11358: F, t42731: F, t52011: F, t60927: F, t63468: F, t916: F, t41330: F, t41332: F, t63474: F, t63476: F, t63478: F, t63480: F, t63482: F, t63485: F, t63488: F, t63491: F) -> (F, F, F, F, F, F) {
    let t63494 = t11354 * t6120 * t2881;
    let t63497 = t41382 * t6113 * t2881;
    let t63500 = t11358 * t6120 * t2881;
    let t63503 = t52011 * t42731 * t60927;
    let t63505 = t916 * t63468;
    let t63509 = -F::cast_from(0.1898925e1_f64) * t63474 - F::cast_from(0.9494625e0_f64) * t63476 - F::cast_from(0.76790625e-1_f64) * t63478 + F::cast_from(0.3071625e0_f64) * t63480 + F::cast_from(0.15358125e0_f64) * t63482 - F::cast_from(0.1898925e1_f64) * t63485 + F::cast_from(0.3071625e0_f64) * t63488 - F::cast_from(0.3560484375e1_f64) * t63491 + F::cast_from(0.142419375e1_f64) * t63494 + F::cast_from(0.1151859375e0_f64) * t63497 - F::cast_from(0.76790625e-1_f64) * t63500 + F::cast_from(0.65725333333333333333e0_f64) * t63503 + F::cast_from(0.1898925e1_f64) * t63505 - F::cast_from(0.13287407407407407408e0_f64) * t41330 - F::cast_from(0.88582716049382716053e-1_f64) * t41332;
    (t63494, t63497, t63500, t63503, t63505, t63509)
}
