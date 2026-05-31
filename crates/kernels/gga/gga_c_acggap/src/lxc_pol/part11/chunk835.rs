//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 835/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk835<F: Float>(t467: F, t560: F, t9097: F, t182: F, t310: F, t129: F, t5: F, t2162: F, t814: F, t2354: F, t813: F, t1077: F, t435: F) -> (F, F, F, F, F, F, F, F) {
    let t9098 = t560 * t467;
    let t9099 = t9097 * t9098;
    let t10098 = t310 * t182;
    let t10146 = t129 * t5;
    let t10409 = t814 * t2162;
    let t10956 = t814 * t2354;
    let t11882 = t813 * t813;
    let t11883 = F::cast_from(1.0_f64) / t11882;
    let t12473 = t435 * t1077;
    (t9098, t9099, t10098, t10146, t10409, t10956, t11883, t12473)
}
