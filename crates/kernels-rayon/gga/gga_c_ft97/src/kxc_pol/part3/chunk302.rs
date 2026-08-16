//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 302/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk302(t1248: f64, t871: f64, t296: f64, t1221: f64, t1225: f64, t1242: f64, t193: f64, t446: f64, t834: f64, t89: f64, t1240: f64, t312: f64) -> (f64, f64, f64, f64) {
    let t1249 = t871 * t1248;
    let t1250 = t296 * t1249;
    let t1253 = -t834 - t446 * t1221 / 9.0_f64 - t446 * t1225 / 3.0_f64 + t89 * t193 * t1242 / 3.0_f64 - t446 * t1250 / 3.0_f64;
    let t1255 = t1240 * t312;
    (t1249, t1250, t1253, t1255)
}
