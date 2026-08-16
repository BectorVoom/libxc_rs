//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1406/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1406(t43503: f64, t43508: f64, t44329: f64, t52687: f64, t52689: f64, t58758: f64, t58761: f64, t58763: f64, t58765: f64, t58770: f64, t58774: f64, t58776: f64, t58778: f64, t58780: f64) -> f64 {
    let t59147 = 0.46074375e0_f64 * t58758 - 0.3560484375e1_f64 * t58761 - 0.28483875e1_f64 * t58763 + 0.1151859375e0_f64 * t58765 - 0.79724444444444444446e0_f64 * t43503 + 0.15944888888888888889e1_f64 * t43508 - 0.54771111111111111111e0_f64 * t44329 + 0.3071625e0_f64 * t58770 + 0.21908444444444444444e0_f64 * t52687 - 0.13145066666666666666e1_f64 * t52689 - 0.379785e1_f64 * t58774 + 0.614325e0_f64 * t58776 + 0.85451625e1_f64 * t58778 - 0.46074375e0_f64 * t58780;
    t59147
}
