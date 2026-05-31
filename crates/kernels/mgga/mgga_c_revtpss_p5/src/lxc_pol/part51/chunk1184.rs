//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1184/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1184<F: Float>(t32298: F, t7898: F, t118: F, t125345: F, t125945: F, t125948: F, t125950: F, t127189: F, t127296: F, t127299: F, t127302: F, t127305: F, t127306: F, t127308: F, t127313: F, t127314: F, t127318: F, t127324: F, t127326: F, t127328: F, t127330: F, t32162: F, t4293: F, t671: F) -> F {
    let t127332 = t7898 * t32298;
    let t127333 = -F::cast_from(4.0_f64) * t125945 - t125948 - t125950 - t118 * (t127189 + t127296) - F::cast_from(2.0_f64) * t127299 + t127302 + t127305 - F::cast_from(2.0_f64) * t127306 + F::cast_from(6.0_f64) * t127308 + t127313 + F::cast_from(2.0_f64) * t127314 + F::cast_from(2.0_f64) * t127318 - F::cast_from(2.0_f64) * t125345 * t671 - F::cast_from(2.0_f64) * t32162 * t4293 - F::cast_from(4.0_f64) * t127324 - F::cast_from(4.0_f64) * t127326 - F::cast_from(4.0_f64) * t127328 - F::cast_from(4.0_f64) * t127330 + t127332;
    t127333
}
