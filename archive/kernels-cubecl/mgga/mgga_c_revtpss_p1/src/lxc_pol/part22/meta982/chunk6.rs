//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3329/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3329<F: Float>(t18838: F, t2411: F, t4537: F, t890: F, t14436: F, t18256: F, t1940: F, t50080: F, t62297: F, t62298: F, t62299: F, t62300: F, t62301: F, t62303: F, t62304: F, t62305: F, t62306: F) -> F {
    let t63160 = t18838 * t2411;
    let t63164 = t4537 * t890;
    let t63170 = F::cast_from(8.0_f64) * t14436 * t1940 * t63164 - F::cast_from(2.0_f64) * t1940 * t63160 * t890 + F::cast_from(12.0_f64) * t18256 * t50080 + t62297 + t62298 - t62299 + t62300 + t62301 + t62303 + t62304 + t62305 + t62306;
    t63170
}
