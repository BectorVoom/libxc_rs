//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 891/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk891(t1017: f64, t3087: f64, t1015: f64, t1012: f64, t2940: f64, t2952: f64, t2928: f64, t320: f64) -> (f64, f64, f64, f64) {
    let t10515 = t3087 * t1017;
    let t10516 = t1015 * t10515;
    let t10517 = t1012 * t10516;
    let t10521 = 0.51947577317044391276e2_f64 * t2940 * t2952;
    let t10523 = 1.0_f64 / t2928 / t320;
    (t10516, t10517, t10521, t10523)
}
