//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 885/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk885<F: Float>(t12261: F, t1592: F, t535: F, t1568: F, t4420: F, t4419: F, t4498: F, t4377: F, t1587: F, t1572: F, t12951: F, t539: F, t13900: F, t1582: F, t1580: F, t3973: F, t4407: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14942 = t12261 * t1592;
    let t14943 = t535 * t14942;
    let t14945 = t1568 * t4420;
    let t14947 = t4419 * t4498;
    let t14948 = t535 * t14947;
    let t14956 = t4419 * t4377;
    let t14957 = t535 * t14956;
    let t14961 = t1587 * t1587;
    let t14962 = 1.0 / t14961;
    let t14983 = t1572 * t4420;
    let t14995 = t539 * t12951;
    let t15005 = t13900 * t1582;
    let t15006 = t1580 * t15005;
    let t15008 = t3973 * t4407;
    (t14943, t14945, t14948, t14957, t14962, t14983, t14995, t15006, t15008)
}
