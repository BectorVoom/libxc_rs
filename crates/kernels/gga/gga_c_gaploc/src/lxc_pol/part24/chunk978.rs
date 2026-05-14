//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 978/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk978<F: Float>(t10912: F, t1422: F, t787: F, t2672: F, t6081: F, t1980: F, t7339: F, t20157: F, t805: F, t831: F, t5558: F, t952: F, t1959: F, t2590: F, t119: F, t19077: F, t481: F) -> (F, F, F, F, F, F, F) {
    let t23477 = t787 * t10912 * t1422;
    let t23492 = t6081 * t2672;
    let t23495 = t1980 * t7339;
    let t23516 = t805 * t831 * t20157;
    let t23555 = t952 * t5558;
    let t23575 = t2590 * t1959;
    let t23609 = t481 * t19077 * t119;
    (t23477, t23492, t23495, t23516, t23555, t23575, t23609)
}
