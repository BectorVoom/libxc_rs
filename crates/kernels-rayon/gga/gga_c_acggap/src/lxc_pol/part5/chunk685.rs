//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 685/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk685(t1427: f64, t721: f64, t4795: f64, t1049: f64, t1483: f64, t346: f64, t4099: f64, t345: f64, t1480: f64, t3111: f64, t1298: f64, t355: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4796 = t1427 * t721;
    let t4797 = t4795 * t4796;
    let t4798 = 0.2445e0_f64 * t4797;
    let t4799 = t1049 * t1483;
    let t4800 = 0.978e0_f64 * t4799;
    let t4801 = t346 * t4099;
    let t4802 = t345 * t4801;
    let t4804 = t3111 * t1480;
    let t4806 = t355 * t1298;
    (t4796, t4797, t4798, t4799, t4800, t4801, t4802, t4804, t4806)
}
