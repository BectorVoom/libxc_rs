//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 645/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk645(t2247: f64, t47: f64, t68: f64, t72: f64, t424: f64, t626: f64, t419: f64, t1570: f64, t23: f64, t10: f64, t3050: f64, t83: f64) -> (f64, f64, f64, f64, f64) {
    let t8076 = t47 * t2247;
    let t8078 = t68 * t8076 * t72;
    let t8079 = 0.70937342644032921812e-2_f64 * t8078;
    let t8109 = t626 * t424;
    let t8110 = t419 * t8109;
    let t8119 = 1.0_f64 / t23 / t1570;
    let t8189 = t10 * t3050 * t83;
    (t8078, t8079, t8110, t8119, t8189)
}
