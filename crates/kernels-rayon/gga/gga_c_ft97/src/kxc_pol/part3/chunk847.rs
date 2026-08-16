//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 847/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk847(t4714: f64, t574: f64, t616: f64, t167: f64, t16919: f64, t1053: f64, t3565: f64, t2179: f64, t144: f64, t4823: f64, t9419: f64, t3408: f64, t920: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17174 = t574 * t616 * t4714;
    let t17178 = t574 * t167 * t16919;
    let t17181 = t1053 * t3565;
    let t17182 = t2179 * t17181;
    let t17183 = t144 * t17182;
    let t17186 = t9419 * t4823;
    let t17189 = t920 * t3408;
    (t17174, t17178, t17182, t17183, t17186, t17189)
}
