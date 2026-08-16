//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1321/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1321(t1: f64, t14300: f64, t16975: f64, t2640: f64, t2722: f64, t297: f64, t313: f64, t40391: f64, t41757: f64, t41832: f64, t51102: f64, t51164: f64, t51169: f64, t56740: f64, t57530: f64, t57537: f64, t57541: f64, t57545: f64, t57554: f64, t7449: f64, t7491: f64, t862: f64, t874: f64, t893: f64) -> f64 {
    let t57561 = 0.23666877659387696117e0_f64 * t2640 * t40391 * t16975 + 0.48295341609937543636e-2_f64 * t41757 + 0.35500316489081544176e-1_f64 * t874 * t313 * t57530 * t1 * t297 + 0.10866451862235947318e0_f64 * t893 * t57537 + 0.90553765518632894319e-2_f64 * t893 * t57541 - 0.96590683219875087274e-1_f64 * t893 * t57545 + 0.18933502127510156893e0_f64 * t51164 + 0.18933502127510156893e0_f64 * t51169 - t41832 / 162.0_f64 + t862 * t2722 * t56740 / 8.0_f64 + 0.36629113921839320676e2_f64 * t7491 * t14300 * t57554 - 0.18314556960919660338e2_f64 * t7449 * t14300 * t51102;
    t57561
}
