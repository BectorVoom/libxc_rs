//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3134/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3134(t24480: f64, t3531: f64, t16784: f64, t6556: f64, t1179: f64, t1188: f64, t1196: f64, t81998: f64, t1187: f64, t24375: f64, t45187: f64, t45190: f64) -> (f64, f64, f64, f64) {
    let t82404 = 0.35089341735807877242e1_f64 * t3531 * t24480;
    let t82406 = 0.51947577317044391276e2_f64 * t16784 * t6556;
    let t82410 = 0.5848223622634646207e0_f64 * t1196 * t1179 * t81998 * t1188;
    let t82415 = 0.91082604192152556044e5_f64 * t1196 * t45187 * t24375 * t45190 * t1187;
    (t82404, t82406, t82410, t82415)
}
