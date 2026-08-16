//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1720/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1720<F: Float>(t1678: F, t989: F, t15654: F, t378: F, t1086: F, t4743: F, t1071: F, t3298: F, t342: F, t3302: F, t4893: F, t359: F, t4930: F) -> (F, F, F, F, F, F, F) {
    let t16371 = t989 * t1678;
    let t16374 = t15654 * t378;
    let t16381 = t4743 * t1086;
    let t16409 = t3298 * t1071;
    let t16410 = t342 * t16409;
    let t16432 = t4893 * t3302;
    let t16449 = t359 * t4930;
    (t16371, t16374, t16381, t16409, t16410, t16432, t16449)
}
