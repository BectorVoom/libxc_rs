//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1064/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1064<F: Float>(t624: F, t922: F, t560: F, t839: F, t10409: F, t1427: F, t15026: F, t2254: F, t2355: F, t24589: F, t2541: F, t29943: F, t29948: F, t29961: F, t32246: F, t32249: F, t4818: F, t4822: F, t5439: F, t567: F, t625: F, t7278: F, t7297: F, t8031: F, t8372: F) -> (F,) {
    let t36621 = t922 * t624;
    let t36647 = t560 * t839;
    let t36654 = -6.0 * t10409 * t5439 * t7297 + 12.0 * t1427 * t29948 * t8372 - t15026 * t567 * t625 + 3.0 * t2254 * t29943 * t567 + 6.0 * t2254 * t36621 * t567 - t2355 * t567 * t8031 - 6.0 * t24589 * t2541 * t7297 - 3.0 * t2541 * t36647 * t7297 + 12.0 * t4818 * t7278 * t8372 + 6.0 * t4822 * t7278 * t8372 + 2.0 * t29961 + 3.0 * t32246 - 2.0 * t32249;
    (t36654,)
}
