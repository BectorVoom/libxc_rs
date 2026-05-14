//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1044/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1044<F: Float>(t119711: F, t125961: F, t125984: F, t126017: F, t126030: F, t18875: F, t1940: F, t2403: F, t25440: F, t25445: F, t27363: F, t27375: F, t27384: F, t31876: F, t4343: F, t4433: F, t4537: F, t4541: F, t7091: F, t7782: F, t8494: F) -> (F,) {
    let t127180 = -6.0 * t119711 * t1940 * t27384 + 4.0 * t125961 * t1940 * t25445 - 6.0 * t125984 * t2403 * t7091 + 4.0 * t126017 * t1940 * t25445 - 6.0 * t126030 * t2403 * t7091 + 6.0 * t18875 * t2403 * t31876 - 2.0 * t1940 * t25440 * t7782 - 2.0 * t1940 * t27363 * t7091 + 2.0 * t1940 * t31876 * t4537 + 6.0 * t2403 * t27375 * t31876 - 3.0 * t2403 * t4343 * t8494 - 6.0 * t4433 * t4541 * t8494;
    (t127180,)
}
