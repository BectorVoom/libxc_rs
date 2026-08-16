//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 947/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk947(t3234: f64, t8469: f64, t2508: f64, t2580: f64, t2958: f64, t9688: f64, t13221: f64, t7129: f64, t2558: f64, t33232: f64, t9647: f64, t13188: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43213 = t8469 * t3234;
    let t43216 = 0.15381052460284448567e-1_f64 * t2508 * t2580 * t43213;
    let t43217 = t2958 * t9688;
    let t43220 = 0.15381052460284448567e-1_f64 * t2508 * t2580 * t43217;
    let t43222 = 0.76905262301422242837e-2_f64 * t7129 * t13221;
    let t43224 = t9647 * t33232 * t2558;
    let t43231 = t7129 * t13188;
    (t43213, t43216, t43217, t43220, t43222, t43224, t43231)
}
