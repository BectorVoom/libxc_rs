//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 687/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk687(t3190: f64, t8392: f64, t3115: f64, t1882: f64, t3257: f64, t110: f64, t1786: f64, t463: f64, t488: f64, t100: f64, t370: f64, t3263: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11436 = 4.0_f64 / 27.0_f64 * t8392 * t3190;
    let t11448 = 2.0_f64 / 27.0_f64 * t8392 * t3115;
    let t11467 = 2.0_f64 / 9.0_f64 * t1882 * t3257;
    let t11468 = t1786 * t110;
    let t11472 = t463 * t488;
    let t11490 = t370 * t100;
    let t11535 = 2.0_f64 / 9.0_f64 * t1882 * t3263;
    (t11436, t11448, t11467, t11468, t11472, t11490, t11535)
}
