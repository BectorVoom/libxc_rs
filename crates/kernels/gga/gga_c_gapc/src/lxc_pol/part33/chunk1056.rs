//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1056/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1056<F: Float>(t169: F, t34159: F, t5486: F, t619: F, t11361: F, t27658: F, t2993: F, t11601: F, t9291: F, t3691: F, t8965: F, t35069: F, t35071: F, t35074: F, t35077: F, t35080: F, t35083: F, t35086: F) -> (F,) {
    let t35090 = t169 * t5486 * t34159 * t619;
    let t35093 = t2993 * t11361 * t27658;
    let t35095 = t11601 * t9291;
    let t35097 = t3691 * t8965;
    let t35099 = 0.13259557375557346398e-6 * t35069 - 0.21103240995305505364e-7 * t35071 - 0.13494357638888888889e-4 * t35074 + 0.28985453471303521737e-5 * t35077 - 0.20241536458333333334e-3 * t35080 - 0.91551759647971344971e-6 * t35083 + 0.16730225092923199692e-7 * t35086 + 0.51491428373437201895e-6 * t35090 - 0.78584976712469872988e-8 * t35093 + 0.13506074236995523433e-5 * t35095 + 0.57970906942607043474e-5 * t35097;
    (t35099,)
}
