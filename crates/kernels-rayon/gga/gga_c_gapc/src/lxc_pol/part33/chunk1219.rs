//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1219/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1219(t11361: f64, t27658: f64, t2993: f64, t11601: f64, t9291: f64, t3691: f64, t8965: f64, t35069: f64, t35071: f64, t35074: f64, t35077: f64, t35080: f64, t35083: f64, t35086: f64, t35090: f64) -> f64 {
    let t35093 = t2993 * t11361 * t27658;
    let t35095 = t11601 * t9291;
    let t35097 = t3691 * t8965;
    let t35099 = 0.13259557375557346398e-6_f64 * t35069 - 0.21103240995305505364e-7_f64 * t35071 - 0.13494357638888888889e-4_f64 * t35074 + 0.28985453471303521737e-5_f64 * t35077 - 0.20241536458333333334e-3_f64 * t35080 - 0.91551759647971344971e-6_f64 * t35083 + 0.16730225092923199692e-7_f64 * t35086 + 0.51491428373437201895e-6_f64 * t35090 - 0.78584976712469872988e-8_f64 * t35093 + 0.13506074236995523433e-5_f64 * t35095 + 0.57970906942607043474e-5_f64 * t35097;
    t35099
}
