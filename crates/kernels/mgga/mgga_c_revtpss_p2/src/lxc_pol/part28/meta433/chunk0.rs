//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1628/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1628<F: Float>(t378: F, t4743: F, t1678: F, t989: F, t15654: F, t1086: F, t1089: F, t15920: F, t16076: F, t12073: F, t1651: F, t1082: F, t16152: F) -> (F, F, F, F, F, F, F, F) {
    let t16362 = t4743 * t378;
    let t16371 = t989 * t1678;
    let t16374 = t15654 * t378;
    let t16381 = t4743 * t1086;
    let t16390 = t15920 * t1089;
    let t16393 = t16076 * t1089;
    let t16396 = t12073 * t1651;
    let t16399 = t1082 * t16152;
    (t16362, t16371, t16374, t16381, t16390, t16393, t16396, t16399)
}
