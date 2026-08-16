//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 839/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk839(t2180: f64, t920: f64, t13165: f64, t2210: f64, t3052: f64, t569: f64, t616: f64, t2142: f64, t3478: f64, t574: f64, t3483: f64, t9276: f64) -> (f64, f64, f64, f64) {
    let t13166 = t920 * t2180;
    let t13167 = t13165 * t13166;
    let t13168 = t2210 * t13167;
    let t13173 = t569 * t616 * t3052;
    let t13177 = t574 * t2142 * t3478;
    let t13180 = t9276 * t3483;
    (t13168, t13173, t13177, t13180)
}
