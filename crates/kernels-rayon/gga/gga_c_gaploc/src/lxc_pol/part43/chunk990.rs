//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 990/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk990(t2508: f64, t2541: f64, t39022: f64, t13918: f64, t7129: f64, t2717: f64, t3722: f64, t12305: f64, t954: f64, t169: f64, t270: f64, t299: f64, t47311: f64, t706: f64) -> (f64, f64, f64, f64, f64) {
    let t47749 = t2508 * t2541 * t39022;
    let t47752 = t7129 * t13918;
    let t47755 = t2508 * t2717 * t3722;
    let t47758 = t2508 * t954 * t12305;
    let t47764 = 0.76905262301422242837e-2_f64 * t270 * t706 * t47311 * t169 * t299;
    (t47749, t47752, t47755, t47758, t47764)
}
