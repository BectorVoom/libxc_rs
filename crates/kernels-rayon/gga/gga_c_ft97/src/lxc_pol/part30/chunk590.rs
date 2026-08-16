//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 590/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk590(t27633: f64, t6035: f64, t1127: f64, t703: f64, t684: f64, t1119: f64, t70: f64, t709: f64, t992: f64, t704: f64, t27499: f64, t3766: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27634 = t6035 * t27633;
    let t27637 = t703 * t1127;
    let t27638 = t27637 * t684;
    let t27642 = t1119 * t70;
    let t27646 = t992 * t709;
    let t27647 = t704 * t27646;
    let t27651 = t3766 * t27499;
    (t27634, t27638, t27642, t27646, t27647, t27651)
}
