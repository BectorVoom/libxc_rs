//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1115/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1115(t70321: f64, t72: f64, t73484: f64, t739: f64, t76036: f64, t78439: f64, t78440: f64, t78444: f64, t78446: f64, t78451: f64, t78454: f64, t78457: f64, t78462: f64, t78464: f64, t78465: f64, t78469: f64, t80402: f64, t80407: f64, t80413: f64, t80421: f64, t80426: f64, t80433: f64, t80442: f64, t80449: f64, t80462: f64, t80466: f64, t80472: f64, t80477: f64, t80478: f64, t80482: f64, t80485: f64, t80489: f64, t80493: f64, t80497: f64, t82: f64, t884: f64) -> f64 {
    let t80509 = -t78439 + t78440 + t78444 + t72 * t82 * (t80407 + t80413 + t80421 + t80426 + t80433 + t80442 + t80449 + t80462 + t80466 + t80472 + t80477 + t80482 + t80485 + t80489 + t80493 + t80497) + t78446 - t78451 - 0.17519306092901367186e-5_f64 * t76036 + t78454 - t78457 - t73484 + t78462 - t70321 - t78464 - t78465 + t78469 - 0.59871208509319042821e-1_f64 * t739 * t80402 + 0.59871208509319042821e-1_f64 * t884 * t80478;
    t80509
}
