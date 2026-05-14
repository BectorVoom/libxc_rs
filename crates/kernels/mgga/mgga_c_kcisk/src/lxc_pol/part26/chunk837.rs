//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 837/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk837<F: Float>(t14942: F, t535: F, t1587: F, t1390: F, t1588: F, t12951: F, t539: F, t13900: F, t1582: F, t1580: F, t13614: F, t397: F, t1572: F, t4416: F, t1607: F, t4534: F) -> (F, F, F, F, F, F, F, F) {
    let t14943 = t535 * t14942;
    let t14961 = t1587 * t1587;
    let t14962 = 1.0 / t14961;
    let t14978 = t1588 * t1390;
    let t14995 = t539 * t12951;
    let t15005 = t13900 * t1582;
    let t15006 = t1580 * t15005;
    let t15050 = t397 * t13614 * t539;
    let t15052 = 0.9994882620098509563e-2 * t535 * t15050;
    let t15064 = t1572 * t4416;
    let t15087 = t1607 * t4534;
    (t14943, t14962, t14978, t14995, t15006, t15052, t15064, t15087)
}
