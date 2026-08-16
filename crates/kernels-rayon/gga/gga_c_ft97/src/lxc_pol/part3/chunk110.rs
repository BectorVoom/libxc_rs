//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 110/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk110(t2: f64, t295: f64, t192: f64, t92: f64, t91: f64, t298: f64) -> (f64, f64, f64, f64, f64) {
    let t302 = t295 * t2;
    let t303 = t192 * t302;
    let t304 = t92 * t303;
    let t305 = f64::sqrt(t304);
    let t306 = t91 * t305;
    let t309 = 3.0_f64 + t306 / 3.0_f64 + t298 / 3.0_f64;
    (t303, t304, t305, t306, t309)
}
