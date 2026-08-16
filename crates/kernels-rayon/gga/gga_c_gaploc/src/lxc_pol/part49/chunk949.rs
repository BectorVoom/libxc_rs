//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 949/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk949(t204: f64, t41749: f64, t587: f64, t41738: f64, t6710: f64, t6711: f64, t6717: f64, t6914: f64, t12943: f64, t4379: f64, t10608: f64, t9272: f64, t9278: f64) -> (f64, f64, f64, f64, f64) {
    let t42309 = 0.18404604457881959845e2_f64 * t587 * t204 * t41749;
    let t42312 = 0.43710935587469654631e2_f64 * t6710 * t6711 * t41738;
    let t42315 = 0.12423108009070322895e3_f64 * t6914 * t6717 * t41749;
    let t42316 = t4379 * t12943;
    let t42349 = t9272 * t10608 * t9278;
    (t42309, t42312, t42315, t42316, t42349)
}
