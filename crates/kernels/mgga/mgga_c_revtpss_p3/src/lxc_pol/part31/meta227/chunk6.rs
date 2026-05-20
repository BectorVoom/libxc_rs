//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1020/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1020<F: Float>(t198: F, t207: F, t2393: F, t2403: F, t2411: F, t2621: F, t5927: F, t5943: F, t5945: F, t5947: F, t5948: F, t5962: F, t5966: F, t5970: F, t6001: F, t6004: F, t6075: F, t6079: F, t765: F, t892: F) -> F {
    let t6083 = -t198 * t207 * t2411 * t6079 + t198 * t207 * t6075 * t892 + F::new(6.0) * t198 * t2393 * t5966 + F::new(3.0) * t198 * t5962 * t765 + F::new(6.0) * t2403 * t5970 + t2621 + t5927 + t5943 + t5945 + t5947 - t5948 + t6001 + t6004;
    t6083
}
