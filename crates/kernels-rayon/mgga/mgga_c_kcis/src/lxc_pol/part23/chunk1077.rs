//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1077/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1077(t27582: f64, t27620: f64, t27645: f64, t27669: f64, t1636: f64, t8010: f64, t27490: f64, t27493: f64, t27496: f64, t27497: f64, t27500: f64, t27505: f64, t27508: f64, t27511: f64, t27554: f64, t4480: f64, t633: f64) -> (f64, f64, f64) {
    let t27671 = t27582 + t27620 + t27645 + t27669;
    let t27673 = t8010 * t1636;
    let t27676 = t27671 * t633 + 4.0_f64 * t27673 * t4480 - t27490 + t27493 - t27496 + t27497 - t27500 + t27505 - t27508 - t27511 + t27554;
    (t27671, t27673, t27676)
}
