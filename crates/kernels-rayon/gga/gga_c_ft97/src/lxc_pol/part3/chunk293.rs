//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 293/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk293(t1168: f64, t762: f64, t242: f64, t1140: f64, t1144: f64, t1162: f64, t193: f64, t446: f64, t723: f64, t89: f64, t1160: f64, t258: f64) -> (f64, f64, f64, f64) {
    let t1169 = t762 * t1168;
    let t1170 = t242 * t1169;
    let t1173 = -t723 - t446 * t1140 / 9.0_f64 - t446 * t1144 / 3.0_f64 + t89 * t193 * t1162 / 3.0_f64 - t446 * t1170 / 3.0_f64;
    let t1175 = t1160 * t258;
    (t1169, t1170, t1173, t1175)
}
