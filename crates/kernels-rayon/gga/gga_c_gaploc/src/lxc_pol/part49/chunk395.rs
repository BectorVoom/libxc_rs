//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 395/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk395(t3255: f64, t738: f64, t270: f64, t3212: f64, t3237: f64, t3242: f64, t3244: f64, t3250: f64, t3252: f64, t977: f64) -> (f64, f64, f64) {
    let t3256 = t738 * t3255;
    let t3259 = 0.76905262301422242837e-2_f64 * t270 * t3212 + 0.76905262301422242837e-2_f64 * t270 * t3237 + t3242 - 0.23071578690426672851e-1_f64 * t270 * t3244 - t3250 + 0.15381052460284448567e-1_f64 * t270 * t3252 - 0.76905262301422242837e-2_f64 * t270 * t3256;
    let t3263 = t977 * t977;
    (t3256, t3259, t3263)
}
