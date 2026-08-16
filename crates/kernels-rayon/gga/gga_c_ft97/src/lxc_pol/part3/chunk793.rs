//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 793/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk793(t3271: f64, t925: f64, t8557: f64, t11468: f64, t15951: f64, t363: f64, t979: f64, t2983: f64, t11556: f64, t11552: f64, t15955: f64, t3214: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16305 = t925 * t3271;
    let t16306 = t8557 * t16305;
    let t16309 = t11468 * t15951;
    let t16312 = t979 * t363;
    let t16313 = t2983 * t16312;
    let t16314 = t11556 * t16313;
    let t16317 = t11552 * t15955;
    let t16320 = t925 * t3214;
    (t16306, t16309, t16312, t16314, t16317, t16320)
}
