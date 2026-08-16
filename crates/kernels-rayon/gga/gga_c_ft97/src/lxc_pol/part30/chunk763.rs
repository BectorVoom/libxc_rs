//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 763/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk763(t2574: f64, t7440: f64, t773: f64, t33605: f64, t33609: f64, t33613: f64, t33617: f64, t33622: f64, t33626: f64, t33630: f64, t33632: f64, t33636: f64, t33638: f64, t33642: f64, t446: f64) -> (f64, f64) {
    let t33646 = t2574 * t773 * t7440;
    let t33649 = 2.0_f64 / 3.0_f64 * t446 * t33605 + 4.0_f64 / 3.0_f64 * t446 * t33609 - t446 * t33613 / 9.0_f64 - 2.0_f64 * t446 * t33617 - 2.0_f64 / 3.0_f64 * t446 * t33622 + 4.0_f64 / 3.0_f64 * t446 * t33626 - t33630 + 4.0_f64 / 3.0_f64 * t446 * t33632 + t33636 - 2.0_f64 / 3.0_f64 * t446 * t33638 - 2.0_f64 / 3.0_f64 * t446 * t33642 + 2.0_f64 / 3.0_f64 * t446 * t33646;
    (t33646, t33649)
}
