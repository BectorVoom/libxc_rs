//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2746/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2746(t14468: f64, t14791: f64, t2477: f64, t2745: f64, t2749: f64, t40455: f64, t40471: f64, t40473: f64, t40475: f64, t40477: f64, t40482: f64, t40484: f64, t40489: f64, t50493: f64, t50497: f64, t50502: f64, t50505: f64, t50511: f64, t50518: f64, t50522: f64, t50524: f64, t50526: f64, t775: f64, t828: f64, t851: f64) -> f64 {
    let t50528 = -0.24098469264142313933e-5_f64 * t40455 + 0.12004725073059526352e0_f64 * t40471 - 0.22866142996303859718e-3_f64 * t40473 - 0.22866142996303859718e-3_f64 * t40475 + 0.16262400898971305031e-2_f64 * t40477 + 0.71456696863449561619e-5_f64 * t40482 + 0.40015750243531754508e-2_f64 * t40484 + 0.21675198048579700358e-2_f64 * t40489 - 0.42874018118069736972e-4_f64 * t50493 + 0.12862205435420921092e-3_f64 * t50497 + 0.42874018118069736972e-3_f64 * t50502 - t50505 + 0.12862205435420921092e-1_f64 * t851 * t2477 * t828 * t14468 * t775 + 0.25724410870841842183e-2_f64 * t2745 * t14791 * t50511 * t2749 + 0.17149607247227894789e-3_f64 * t50518 - 0.12862205435420921092e-3_f64 * t50522 + 0.34013387707001991331e0_f64 * t50524 - 0.12004725073059526352e0_f64 * t50526;
    t50528
}
