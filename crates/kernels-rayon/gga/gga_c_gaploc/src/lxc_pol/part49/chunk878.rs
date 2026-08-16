//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 878/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk878(t6125: f64, t883: f64, t123: f64, t28002: f64, t9647: f64, t16880: f64, t28669: f64, t28924: f64, t5539: f64, t286: f64, t39622: f64, t708: f64) -> (f64, f64, f64, f64) {
    let t40594 = t883 * t6125;
    let t40596 = t9647 * t28002 * t123 * t40594;
    let t40599 = t9647 * t16880 * t28669;
    let t40602 = t9647 * t5539 * t28924;
    let t40612 = t39622 * t286 * t708;
    (t40596, t40599, t40602, t40612)
}
