//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1023/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1023<F: Float>(t146: F, t19875: F, t252: F, t545: F, t6394: F, t1415: F, t57: F, t2252: F, t6212: F, t1234: F, t19790: F, t560: F) -> (F, F, F, F, F, F) {
    let t19877 = t146 * t19875 * t252;
    let t19883 = t545 * t6394;
    let t20094 = t1415 * t57;
    let t20102 = t6212 * t2252;
    let t20132 = t6212 * t1234;
    let t20146 = t19790 * t560;
    (t19877, t19883, t20094, t20102, t20132, t20146)
}
