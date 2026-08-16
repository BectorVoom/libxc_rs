//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1417/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1417(t43503: f64, t43508: f64, t44329: f64, t52687: f64, t52689: f64, t58758: f64, t58761: f64, t58763: f64, t58765: f64, t58770: f64, t58774: f64, t58776: f64, t58778: f64, t58780: f64) -> f64 {
    let t59325 = 0.94674375e0_f64 * t58758 - 0.6618234375e1_f64 * t58761 - 0.52945875e1_f64 * t58763 + 0.2366859375e0_f64 * t58765 - 0.13772666666666666666e1_f64 * t43503 + 0.27545333333333333333e1_f64 * t43508 - 0.69463333333333333334e0_f64 * t44329 + 0.6311625e0_f64 * t58770 + 0.27785333333333333333e0_f64 * t52687 - 0.166712e1_f64 * t52689 - 0.705945e1_f64 * t58774 + 0.1262325e1_f64 * t58776 + 0.158837625e2_f64 * t58778 - 0.94674375e0_f64 * t58780;
    t59325
}
