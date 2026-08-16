//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1269/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1269<F: Float>(t11144: F, t11852: F, t15688: F, t3299: F, t1043: F, t905: F, t606: F, t3155: F, t3057: F, t379: F, t1071: F, t3298: F) -> (F, F, F, F, F) {
    let t16208 = t11852 * t11144;
    let t16226 = t3299 * t15688;
    let t16227 = t1043 * t905;
    let t16228 = t16227 * t606;
    let t16229 = t3155 * t16228;
    let t16312 = t3057 * t379;
    let t16409 = t3298 * t1071;
    (t16208, t16226, t16229, t16312, t16409)
}
