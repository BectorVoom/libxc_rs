//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 980/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk980(t38568: f64, t4669: f64, t27041: f64, t38798: f64, t25820: f64, t38801: f64, t25877: f64, t38792: f64, t38795: f64, t1587: f64, t2064: f64, t793: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41536 = t4669 * t38568;
    let t41538 = t27041 * t38798;
    let t41540 = t25820 * t38801;
    let t41542 = t25877 * t38792;
    let t41544 = t25820 * t38795;
    let t41548 = t2064 * t1587;
    let t41549 = t793 * t41548;
    (t41536, t41538, t41540, t41542, t41544, t41548, t41549)
}
