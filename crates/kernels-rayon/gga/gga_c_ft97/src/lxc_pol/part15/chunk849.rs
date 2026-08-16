//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 849/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk849(t22216: f64, t22258: f64, t22396: f64, t22467: f64, t22439: f64, t312: f64, t1218: f64, t1253: f64, t21931: f64, t21933: f64, t22168: f64, t22250: f64, t22347: f64, t22356: f64, t22360: f64, t22406: f64, t22464: f64, t301: f64, t317: f64, t5207: f64, t5305: f64, t5422: f64) -> (f64, f64, f64) {
    let t22469 = t22216 + t22258 + t22396 + t22467;
    let t22471 = t22439 * t312;
    let t22479 = -3.0_f64 * t1218 * t5422 - 3.0_f64 * t1253 * t5207 - 3.0_f64 * t1253 * t5305 - t21931 * t317 - 2.0_f64 * t21933 * t317 - t22168 * t317 - t22469 * t301 - 12.0_f64 * t22250 - 2.0_f64 * t22347 + 12.0_f64 * t22356 - 6.0_f64 * t22360 + 12.0_f64 * t22406 - 6.0_f64 * t22464 + 2.0_f64 * t22471;
    (t22469, t22471, t22479)
}
