//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 861/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk861(t45320: f64, t7427: f64, t7573: f64, t2615: f64, t326: f64, t45305: f64, t11603: f64, t2464: f64, t2465: f64, t13638: f64, t7416: f64, t11627: f64, t2684: f64) -> (f64, f64, f64, f64, f64) {
    let t45323 = 0.12423108009070322895e3_f64 * t7427 * t7573 * t45320;
    let t45326 = 0.46011511144704899612e1_f64 * t2615 * t326 * t45305;
    let t45329 = t7427 * t2464 * t2465 * t11603;
    let t45331 = t7416 * t13638;
    let t45335 = t2684 * t2464 * t2465 * t11627;
    (t45323, t45326, t45329, t45331, t45335)
}
