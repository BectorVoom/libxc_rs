//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 871/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk871(t30334: f64, t544: f64, t9287: f64, t9291: f64, t9562: f64, t20556: f64, t587: f64, t9438: f64, t20967: f64, t12454: f64, t4391: f64, t549: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40251 = t544 * t30334;
    let t40252 = t40251 * t9287;
    let t40258 = t9291 * t9562;
    let t40261 = t587 * t9438 * t20556;
    let t40277 = t9291 * t20967;
    let t40280 = t4391 * t549 * t12454;
    (t40251, t40252, t40258, t40261, t40277, t40280)
}
