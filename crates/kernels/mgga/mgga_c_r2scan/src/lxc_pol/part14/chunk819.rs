//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 819/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk819<F: Float>(t7321: F, t7322: F, t2634: F, t495: F, t5109: F, t2654: F, t1568: F, t2123: F, t1569: F, t920: F) -> (F, F, F, F, F, F, F, F) {
    let t7323 = t7321 * t7322;
    let t7326 = t2634 * t495;
    let t7327 = t5109 * t7326;
    let t7330 = t5109 * t7322;
    let t7333 = t2654 * t495;
    let t7334 = t5109 * t7333;
    let t7337 = t2123 * t1568;
    let t7338 = t920 * t1569;
    (t7323, t7326, t7327, t7330, t7333, t7334, t7337, t7338)
}
