//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 447/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk447<F: Float>(t122: F, t2096: F, t39: F, t500: F, t162: F, t9: F, t267: F, t57: F) -> (F, F, F, F, F, F) {
    let t2097 = t2096 * t122;
    let t2098 = t39 * t500;
    let t2099 = t162 * t2098;
    let t2101 = 1.0 / t9 / t2099;
    let t2102 = t2097 * t2101;
    let t2104 = t267 * t57;
    (t2097, t2098, t2099, t2101, t2102, t2104)
}
