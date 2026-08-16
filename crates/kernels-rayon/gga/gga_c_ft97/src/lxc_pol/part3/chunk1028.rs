//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1028/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1028(t19782: f64, t312: f64, t19329: f64, t19334: f64, t19345: f64, t19379: f64, t19383: f64, t19391: f64, t19431: f64, t19436: f64, t19810: f64, t19863: f64) -> f64 {
    let t19886 = t19782 * t312;
    let t19898 = 2.0_f64 * t19886 - 2.0_f64 * t19334 - 4.0_f64 * t19345 + 8.0_f64 * t19383 - 4.0_f64 * t19329 + 4.0_f64 * t19810 - 12.0_f64 * t19431 + 8.0_f64 * t19436 - 2.0_f64 * t19391 + 4.0_f64 * t19379 - 2.0_f64 * t19863;
    t19898
}
