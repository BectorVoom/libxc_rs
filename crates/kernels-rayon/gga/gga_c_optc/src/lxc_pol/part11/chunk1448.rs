//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1448/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1448(t15008: f64, t15122: f64, t1554: f64, t3980: f64, t47639: f64, t47654: f64, t47659: f64, t5233: f64, t53510: f64, t53909: f64, t5434: f64, t55795: f64, t55797: f64, t59214: f64, t59218: f64) -> f64 {
    let t60275 = 32.0_f64 / 3.0_f64 * t15008 * t5233 - 16.0_f64 / 9.0_f64 * t53909 - 2.0_f64 / 9.0_f64 * t47639 + 0.31013857721884116596e-1_f64 * t3980 * t15122 * t5434 - 176.0_f64 / 27.0_f64 * t47654 + 2.0_f64 / 3.0_f64 * t55795 - 16.0_f64 / 3.0_f64 * t55797 - t59214 - t59218 - 4.0_f64 / 9.0_f64 * t47659 - 0.10337952573961372198e-1_f64 * t3980 * t53510 * t1554;
    t60275
}
