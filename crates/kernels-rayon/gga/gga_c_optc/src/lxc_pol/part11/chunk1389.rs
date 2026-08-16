//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1389/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1389(t14992: f64, t5133: f64, t43503: f64, t43508: f64, t44329: f64, t52687: f64, t52689: f64, t58758: f64, t58761: f64, t58763: f64, t58765: f64, t58770: f64, t58774: f64, t58776: f64, t58778: f64) -> (f64, f64) {
    let t58780 = t14992 * t5133;
    let t58782 = 0.247573125e0_f64 * t58758 - 0.485484375e1_f64 * t58761 - 0.3883875e1_f64 * t58763 + 0.6189328125e-1_f64 * t58765 - 0.80513333333333333336e0_f64 * t43503 + 0.16102666666666666667e1_f64 * t43508 - 0.5519e0_f64 * t44329 + 0.16504875e0_f64 * t58770 + 0.22076e0_f64 * t52687 - 0.132456e1_f64 * t52689 - 0.51785e1_f64 * t58774 + 0.3300975e0_f64 * t58776 + 0.11651625e2_f64 * t58778 - 0.247573125e0_f64 * t58780;
    (t58780, t58782)
}
