//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 964/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk964<F: Float>(t1266: F, t512: F, t57: F, t1607: F, t10856: F, t6271: F, t1615: F, t3320: F, t774: F, t783: F, t1575: F, t2096: F, t571: F, t10710: F, t20665: F, t3342: F, t572: F) -> (F, F, F, F, F, F) {
    let t37699 = t512 * t1266 * t57;
    let t37700 = t37699 * t1607;
    let t37702 = t10856 * t6271;
    let t37707 = t783 * t774 * t1615 * t3320;
    let t37712 = t571 * t1575 * t2096;
    let t37714 = t37712 * t10710 * t20665;
    let t37716 = t572 * t3342;
    (t37699, t37700, t37702, t37707, t37714, t37716)
}
