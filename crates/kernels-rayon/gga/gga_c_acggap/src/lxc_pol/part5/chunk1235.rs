//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1235/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1235(t16288: f64, t16292: f64, t16294: f64, t16300: f64, t16304: f64, t21737: f64, t21740: f64, t21743: f64, t21745: f64, t21747: f64, t21751: f64, t21755: f64, t21759: f64, t21762: f64) -> f64 {
    let t22605 = -t21737 / 2.0_f64 + t21740 / 3.0_f64 + t21743 / 6.0_f64 - t21745 / 3.0_f64 + t21747 / 6.0_f64 + t21751 / 6.0_f64 - t21755 / 12.0_f64 - t21759 / 12.0_f64 - 6.0_f64 * t21762 + 4.0_f64 / 3.0_f64 * t16288 - t16292 / 3.0_f64 - 40.0_f64 / 27.0_f64 * t16294 + 2.0_f64 / 3.0_f64 * t16300 - t16304 / 12.0_f64;
    t22605
}
