//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1158/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1158(t43055: f64, t43087: f64, t43090: f64, t43094: f64, t43096: f64, t43099: f64, t47640: f64, t47644: f64, t47646: f64, t47650: f64, t47652: f64, t2508: f64, t47326: f64, t740: f64) -> (f64, f64) {
    let t47656 = -t43055 + 0.10254034973522965712e-1_f64 * t47640 + t47644 + 0.23071578690426672851e-1_f64 * t47646 - 0.15381052460284448567e-1_f64 * t47650 + 0.42725145723012357132e-3_f64 * t47652 + 0.76905262301422242837e-2_f64 * t43087 + 0.32043859292259267849e-3_f64 * t43090 + t43094 - t43096 + t43099;
    let t47661 = 0.23071578690426672851e-1_f64 * t2508 * t47326 * t740;
    (t47656, t47661)
}
