//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 659/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk659(t9437: f64, t157: f64, t1882: f64, t2182: f64, t2187: f64, t2202: f64, t161: f64, t7943: f64, t89: f64, t2252: f64, t342: f64, t657: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9438 = 1.0_f64 / t9437;
    let t9439 = t157 * t9438;
    let t9449 = t1882 * t2182;
    let t9451 = t1882 * t2187;
    let t9453 = t1882 * t2202;
    let t9457 = 28.0_f64 / 81.0_f64 * t89 * t7943 * t161;
    let t9482 = t342 * t2252 * t657 / 18.0_f64;
    (t9439, t9449, t9451, t9453, t9457, t9482)
}
