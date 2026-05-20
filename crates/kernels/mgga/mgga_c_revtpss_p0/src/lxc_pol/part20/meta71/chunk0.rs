//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 463/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk463<F: Float>(t1455: F, t117: F, t670: F, t572: F, t573: F, t76: F, t84: F, t198: F, t207: F, t159: F, t215: F, t10: F, t17: F, param_d: F) -> (F, F, F, F, F, F, F) {
    let t1459 = param_d * t1455;
    let t1461 = t117 * t670;
    let t1464 = t1459 * t573 + F::new(3.0) * t1461 * t572;
    let t1927 = t76 * t84;
    let t1940 = t198 * t207;
    let t1941 = t215 * t159;
    let t2219 = F::new(2.0) * t10 * t17;
    (t1459, t1461, t1464, t1927, t1940, t1941, t2219)
}
