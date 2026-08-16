//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 511/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk511(t912: f64, t9278: f64, t587: f64, t549: f64, t9194: f64, t544: f64, t6603: f64, t107: f64, t90: f64, t2321: f64) -> (f64, f64, f64, f64, f64) {
    let t9279 = t912 * t9278;
    let t9280 = t587 * t9279;
    let t9281 = 0.38342925953920749676e0_f64 * t9280;
    let t9282 = t549 * t9194;
    let t9285 = t544 * t6603;
    let t9286 = t107 * t90;
    let t9287 = t9286 * t2321;
    (t9281, t9282, t9285, t9286, t9287)
}
