//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 449/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk449(t452: f64, t4572: f64, t488: f64, t1812: f64, t2981: f64, t3006: f64, t4420: f64, t4424: f64, t4428: f64, t4434: f64, t4439: f64, t4498: f64, t4507: f64, t4535: f64) -> (f64, f64) {
    let t4574 = t452 * t488 * t4572;
    let t4589 = -t4507 / 4.0_f64 + t4535 / 2.0_f64 + t1812 + 2.0_f64 / 9.0_f64 * t2981 + 2.0_f64 / 3.0_f64 * t3006 - 2.0_f64 / 9.0_f64 * t4420 + 2.0_f64 / 3.0_f64 * t4424 + 2.0_f64 / 3.0_f64 * t4428 - t4434 / 3.0_f64 + 2.0_f64 * t4439 - t4498;
    (t4574, t4589)
}
