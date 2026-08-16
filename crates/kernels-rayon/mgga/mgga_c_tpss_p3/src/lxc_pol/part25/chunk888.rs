//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 888/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk888(t2349: f64, t8226: f64, t2192: f64, t737: f64, t2348: f64, t2345: f64, t651: f64, t767: f64, t230: f64, t2162: f64, t226: f64, t2376: f64, t339: f64, t769: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8227 = t8226 * t2349;
    let t8229 = t2192 * t737;
    let t8231 = 0.21687162600603479684e-1_f64 * t2348 * t8229;
    let t8232 = t651 * t2345;
    let t8234 = 0.32530743900905219526e-1_f64 * t2348 * t8232;
    let t8274 = t767 * t767;
    let t8275 = 1.0_f64 / t8274;
    let t8276 = t8275 * t230;
    let t8279 = t2162 * t226;
    let t8286 = t339 * t769 * t2376;
    (t8227, t8229, t8231, t8232, t8234, t8275, t8276, t8279, t8286)
}
