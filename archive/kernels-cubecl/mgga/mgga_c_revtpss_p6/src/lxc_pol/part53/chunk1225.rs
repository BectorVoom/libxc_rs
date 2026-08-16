//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1225/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1225<F: Float>(t104115: F, t1937: F, t111734: F, t29427: F, t6993: F, t125938: F, t125942: F, t125945: F, t125948: F, t125950: F, t127299: F, t127302: F, t127305: F, t127306: F, t127308: F) -> F {
    let t129414 = t104115 * t1937;
    let t129416 = t111734 * t1937;
    let t129418 = t29427 * t6993;
    let t129421 = t125938 + t125942 - F::cast_from(2.0_f64) * t125945 - t125948 - t125950 - F::cast_from(2.0_f64) * t129414 - F::cast_from(2.0_f64) * t129416 - F::cast_from(2.0_f64) * t129418 - t127299 + t127302 + t127305 - t127306 + F::cast_from(3.0_f64) * t127308;
    t129421
}
