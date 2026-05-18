//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1124/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1124<F: Float>(t10781: F, t7970: F, t2553: F, t37764: F, t11693: F, t6205: F, t7373: F, t10776: F, t3308: F, t7990: F, t1058: F, t1060: F, t2201: F, t7290: F) -> (F, F, F, F, F, F) {
    let t39577 = t10781 * t7970;
    let t39579 = t37764 * t2553;
    let t39581 = t6205 * t11693;
    let t39583 = t10781 * t7373;
    let t39586 = t10776 * t3308 * t7990;
    let t39599 = t2201 * t1058 * t1060 * t7290;
    (t39577, t39579, t39581, t39583, t39586, t39599)
}
