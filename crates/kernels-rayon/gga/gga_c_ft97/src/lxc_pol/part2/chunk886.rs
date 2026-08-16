//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 886/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk886(t13753: f64, t2601: f64, t3712: f64, t9770: f64, t446: f64, t13728: f64, t13732: f64, t13736: f64, t13740: f64, t13743: f64, t13747: f64, t13750: f64, t9765: f64, t9768: f64) -> (f64, f64, f64) {
    let t13754 = 2.0_f64 / 9.0_f64 * t13753;
    let t13757 = t3712 * t2601;
    let t13758 = t9770 * t13757;
    let t13759 = t446 * t13758;
    let t13761 = 4.0_f64 / 3.0_f64 * t13728 - 22.0_f64 / 27.0_f64 * t13732 + 2.0_f64 / 9.0_f64 * t13736 - t13740 + 2.0_f64 / 3.0_f64 * t13743 - t13747 - t13750 / 3.0_f64 + t13754 - 2.0_f64 / 27.0_f64 * t9768 - 2.0_f64 / 27.0_f64 * t9765 - 4.0_f64 / 9.0_f64 * t13759;
    (t13757, t13759, t13761)
}
