//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 614/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk614(t258: f64, t5132: f64, t1137: f64, t1173: f64, t247: f64, t263: f64, t4915: f64, t5059: f64, t5065: f64, t5148: f64, t5152: f64, t5179: f64) -> (f64, f64) {
    let t5181 = t5132 * t258;
    let t5186 = -2.0_f64 * t1137 * t1173 - t247 * t5179 - t263 * t4915 - t263 * t5059 + 4.0_f64 * t5065 - 2.0_f64 * t5148 - 4.0_f64 * t5152 + 2.0_f64 * t5181;
    (t5181, t5186)
}
