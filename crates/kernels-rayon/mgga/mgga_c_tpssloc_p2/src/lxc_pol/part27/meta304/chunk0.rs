//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1367/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1367(t376: f64, t676: f64, t1023: f64, t248: f64, t1020: f64, t1017: f64, t3087: f64, t1015: f64, t1012: f64, t2928: f64, t320: f64, t10294: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10508 = t676 * t376;
    let t10510 = t248 * t10508 * t1023;
    let t10511 = t1020 * t10510;
    let t10515 = t3087 * t1017;
    let t10516 = t1015 * t10515;
    let t10517 = t1012 * t10516;
    let t10523 = 1.0_f64 / t2928 / t320;
    let t10542 = 0.36793333333333333333e0_f64 * t10294;
    (t10508, t10510, t10511, t10515, t10517, t10523, t10542)
}
