//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 600/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk600(t4129: f64, t799: f64, t27: f64, t89: f64, t2653: f64, t2655: f64, t2658: f64, t4032: f64, t4035: f64, t4039: f64, t4042: f64, t4046: f64, t4049: f64, t4054: f64, t4059: f64) -> (f64, f64, f64) {
    let t4130 = t799 * t4129;
    let t4132 = t89 * t27 * t4130;
    let t4134 = t2653 + t2655 / 54.0_f64 + t2658 / 18.0_f64 + t4032 / 54.0_f64 - t4035 / 27.0_f64 + t4039 / 18.0_f64 + t4042 / 9.0_f64 + t4046 / 9.0_f64 + t4049 / 18.0_f64 + t4054 / 18.0_f64 + t4059 / 3.0_f64 - t4132 / 6.0_f64;
    (t4130, t4132, t4134)
}
