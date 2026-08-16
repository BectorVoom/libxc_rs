//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1201/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1201(t2508: f64, t2580: f64, t32223: f64, t21556: f64, t3420: f64, t10773: f64, t7137: f64, t3448: f64, t24487: f64, t948: f64, t2586: f64, t8637: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32226 = 0.15381052460284448567e-1_f64 * t2508 * t2580 * t32223;
    let t32241 = 0.20508069947045931424e-1_f64 * t21556 * t3420;
    let t32243 = 0.20508069947045931424e-1_f64 * t7137 * t10773;
    let t32245 = 0.41016139894091862846e-1_f64 * t21556 * t3448;
    let t32253 = 0.23071578690426672851e-1_f64 * t2508 * t24487 * t948;
    let t32256 = 0.46143157380853345702e-1_f64 * t2508 * t8637 * t2586;
    (t32226, t32241, t32243, t32245, t32253, t32256)
}
