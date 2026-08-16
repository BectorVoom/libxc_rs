//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1049/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1049(t26760: f64, t27788: f64, t1092: f64, t1133: f64, t14649: f64, t7718: f64, t26671: f64, t8047: f64, t1020: f64, t2822: f64, t8048: f64, t14443: f64, t8037: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27789 = t26760 * t27788;
    let t27790 = t1092 * t27789;
    let t27792 = t14649 * t1133;
    let t27793 = t7718 * t27792;
    let t27794 = t1092 * t27793;
    let t27796 = t26671 * t8047;
    let t27797 = t1020 * t27796;
    let t27799 = t2822 * t8048;
    let t27803 = t14443 * t8037;
    (t27789, t27790, t27792, t27793, t27794, t27796, t27797, t27799, t27803)
}
