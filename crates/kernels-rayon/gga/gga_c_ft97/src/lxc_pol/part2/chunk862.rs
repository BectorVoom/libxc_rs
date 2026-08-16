//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 862/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk862(t13406: f64, t200: f64, t1609: f64, t2378: f64, t223: f64, t9542: f64, t4952: f64, t6783: f64, t2455: f64, t3780: f64, t1127: f64, t2427: f64) -> (f64, f64, f64, f64, f64) {
    let t13407 = t13406 * t200;
    let t13411 = t1609 * t2378;
    let t13412 = t9542 * t223;
    let t13413 = t13411 * t13412;
    let t13414 = t6783 * t4952;
    let t13417 = t3780 * t2455;
    let t13421 = t2427 * t1127;
    (t13407, t13413, t13414, t13417, t13421)
}
