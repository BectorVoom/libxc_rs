//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 725/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk725(t2253: f64, t4359: f64, t12170: f64, t4347: f64, t1263: f64, t8640: f64, t1270: f64, t4372: f64, t4339: f64, t8675: f64, t4343: f64, t4335: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14423 = 2.0_f64 * t2253 * t4359;
    let t14429 = t12170 * t4347;
    let t14431 = t8640 * t1263;
    let t14445 = t8640 * t1270;
    let t14448 = 2.0_f64 / 3.0_f64 * t2253 * t4372;
    let t14478 = 4.0_f64 / 9.0_f64 * t8675 * t4339;
    let t14480 = 4.0_f64 / 9.0_f64 * t8675 * t4343;
    let t14482 = 2.0_f64 / 27.0_f64 * t8675 * t4335;
    (t14423, t14429, t14431, t14445, t14448, t14478, t14480, t14482)
}
