//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1411/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1411(t12168: f64, t17751: f64, t12366: f64, t17748: f64, t5186: f64, t5190: f64, t8582: f64, t1459: f64, t17423: f64, t2993: f64, t3018: f64, t53039: f64) -> (f64, f64, f64, f64, f64) {
    let t59191 = 24.0_f64 * t12168 * t17751;
    let t59193 = 0.19298189186581325787e3_f64 * t12366 * t17748;
    let t59196 = 0.57894567559743977359e3_f64 * t8582 * t5190 * t5186;
    let t59199 = 8.0_f64 * t2993 * t17423 * t1459;
    let t59202 = 0.64327297288604419288e2_f64 * t3018 * t53039 * t1459;
    (t59191, t59193, t59196, t59199, t59202)
}
