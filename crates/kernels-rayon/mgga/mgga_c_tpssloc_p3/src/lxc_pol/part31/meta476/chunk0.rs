//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1637/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1637(t1339: f64, t26297: f64, t22827: f64, t1307: f64, t1825: f64, t22833: f64, t5259: f64, t22759: f64, t242: f64, t1336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26298 = t1339 * t26297;
    let t26299 = t22827 * t26298;
    let t26301 = t1825 * t1307;
    let t26302 = t1339 * t26301;
    let t26303 = t22827 * t26302;
    let t26306 = t22833 * t5259;
    let t26308 = t22759 * t242;
    let t26309 = t1336 * t26308;
    (t26298, t26299, t26301, t26302, t26303, t26306, t26308, t26309)
}
