//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2436/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2436(t10401: f64, t10935: f64, t3186: f64, t3200: f64, t11051: f64, t3069: f64, t3036: f64, t3087: f64, t3033: f64, t3128: f64, t10402: f64, t11034: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42504 = t10935 * t10401;
    let t42505 = t3186 * t42504;
    let t42508 = t3200 * t42504;
    let t42511 = t11051 * t3069;
    let t42520 = t3087 * t3036;
    let t42522 = t3033 * t3128 * t42520;
    let t42541 = t11034 * t10402;
    (t42505, t42508, t42511, t42520, t42522, t42541)
}
