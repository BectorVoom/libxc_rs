//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 703/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk703(t1447: f64, t40: f64, t31: f64, t1450: f64, t459: f64, t1466: f64, t1418: f64, t1426: f64, t1432: f64, t1453: f64, t16: f64, t34: f64, t38: f64, t441: f64, t454: f64, t4796: f64, t4800: f64, t4806: f64, t4812: f64, t4816: f64, t4820: f64, tau0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4827 = 1.0_f64 / t1447 / t40;
    let t4828 = t31 * t4827;
    let t4829 = t1450 * t459;
    let t4832 = t459 * t1466;
    let t4835 = tau0 * t1418;
    let t4856 = -440.0_f64 / 27.0_f64 * t4835 * t16 + 200.0_f64 / 9.0_f64 * t1453 * t441 - 50.0_f64 / 9.0_f64 * t454 * t1426 - 25.0_f64 / 3.0_f64 * t454 * t1432 - 10.0_f64 / 27.0_f64 * t34 * t4796 + 10.0_f64 / 3.0_f64 * t34 * t4800 + 5.0_f64 / 3.0_f64 * t34 * t4806 - 10.0_f64 / 27.0_f64 * t38 * t4812 + 10.0_f64 / 3.0_f64 * t38 * t4816 + 5.0_f64 / 3.0_f64 * t38 * t4820;
    (t4827, t4828, t4829, t4832, t4835, t4856)
}
