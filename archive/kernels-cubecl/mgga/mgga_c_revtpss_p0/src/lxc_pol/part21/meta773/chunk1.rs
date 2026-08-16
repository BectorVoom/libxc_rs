//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2746/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2746<F: Float>(t14468: F, t14791: F, t2477: F, t2745: F, t2749: F, t40455: F, t40471: F, t40473: F, t40475: F, t40477: F, t40482: F, t40484: F, t40489: F, t50493: F, t50497: F, t50502: F, t50505: F, t50511: F, t50518: F, t50522: F, t50524: F, t50526: F, t775: F, t828: F, t851: F) -> F {
    let t50528 = -F::cast_from(0.24098469264142313933e-5_f64) * t40455 + F::cast_from(0.12004725073059526352e0_f64) * t40471 - F::cast_from(0.22866142996303859718e-3_f64) * t40473 - F::cast_from(0.22866142996303859718e-3_f64) * t40475 + F::cast_from(0.16262400898971305031e-2_f64) * t40477 + F::cast_from(0.71456696863449561619e-5_f64) * t40482 + F::cast_from(0.40015750243531754508e-2_f64) * t40484 + F::cast_from(0.21675198048579700358e-2_f64) * t40489 - F::cast_from(0.42874018118069736972e-4_f64) * t50493 + F::cast_from(0.12862205435420921092e-3_f64) * t50497 + F::cast_from(0.42874018118069736972e-3_f64) * t50502 - t50505 + F::cast_from(0.12862205435420921092e-1_f64) * t851 * t2477 * t828 * t14468 * t775 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t14791 * t50511 * t2749 + F::cast_from(0.17149607247227894789e-3_f64) * t50518 - F::cast_from(0.12862205435420921092e-3_f64) * t50522 + F::cast_from(0.34013387707001991331e0_f64) * t50524 - F::cast_from(0.12004725073059526352e0_f64) * t50526;
    t50528
}
