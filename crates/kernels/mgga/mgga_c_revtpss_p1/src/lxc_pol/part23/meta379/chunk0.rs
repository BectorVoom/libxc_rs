//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1719/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1719<F: Float>(t4930: F, t994: F, t1678: F, t3046: F, t3057: F, t379: F, t1078: F, t1651: F, t342: F, t1071: F, t1647: F, t378: F, t4743: F) -> (F, F, F, F, F, F, F) {
    let t16302 = t994 * t4930;
    let t16305 = t3046 * t1678;
    let t16312 = t3057 * t379;
    let t16313 = t1078 * t1651;
    let t16333 = t342 * t4930;
    let t16340 = t1647 * t1071;
    let t16362 = t4743 * t378;
    (t16302, t16305, t16312, t16313, t16333, t16340, t16362)
}
