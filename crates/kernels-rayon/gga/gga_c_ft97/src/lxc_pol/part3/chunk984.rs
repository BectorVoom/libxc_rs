//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 984/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk984(t292: f64, t19099: f64, t19238: f64, t799: f64, t27: f64, t89: f64, t375: f64, t5300: f64, t5226: f64, t17727: f64, t835: f64, t446: f64, t17732: f64, t2857: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t293 = 0.1e-59_f64 < t292;
    let t19240 = piecewise3(t293, t19099 + t19238, 0.0_f64);
    let t19241 = t799 * t19240;
    let t19243 = t89 * t27 * t19241;
    let t19246 = t89 * t375 * t5300;
    let t19249 = t89 * t375 * t5226;
    let t19251 = t835 * t17727;
    let t19252 = t446 * t19251;
    let t19254 = t2857 * t17732;
    (t19240, t19243, t19246, t19249, t19252, t19254)
}
