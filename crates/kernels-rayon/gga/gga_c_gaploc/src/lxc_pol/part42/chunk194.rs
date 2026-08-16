//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 194/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk194(t286: f64, t708: f64, t860: f64, t130: f64, t713: f64, t139: f64, t458: f64, t295: f64, t871: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t924 = t860 * t286 * t708;
    let t926 = t713 * t130;
    let t928 = t139 * t286 * t458;
    let t929 = t926 * t928;
    let t931 = 3.0_f64 / 128.0_f64 * t924 - t929 / 128.0_f64;
    let t933 = t295 * t871;
    let t935 = t931 * t471 + t933 / 2.0_f64;
    (t924, t926, t928, t929, t931, t933, t935)
}
