//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1084/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1084<F: Float>(t25048: F, t3: F, t1518: F, t5883: F, t5801: F, t5920: F, t117: F, t22633: F, t1916: F, t1918: F, t572: F, t573: F, t6941: F, t6945: F, t6948: F, param_d: F) -> (F, F, F, F, F, F) {
    let t25049 = t3 * t25048;
    let t25055 = param_d * t25048;
    let t25063 = t5883 * t1518;
    let t25066 = t5801 * t5920;
    let t25069 = t117 * t22633;
    let t25072 = F::new(18.0) * t1916 * t6945 + F::new(9.0) * t1916 * t6948 + F::new(9.0) * t1918 * t6941 + t25055 * t573 + F::new(6.0) * t25063 * t572 + F::new(18.0) * t25066 * t572 + F::new(3.0) * t25069 * t572;
    (t25049, t25055, t25063, t25066, t25069, t25072)
}
