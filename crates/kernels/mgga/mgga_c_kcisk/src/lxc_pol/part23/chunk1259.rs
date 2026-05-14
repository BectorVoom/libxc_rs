//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1259/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1259<F: Float>(t15093: F, t2339: F, t4534: F, t6602: F, t2168: F, t3961: F, t1322: F, t220: F, t6211: F, t1591: F, t6581: F, t2326: F, t4497: F, t31861: F, t31863: F, t31865: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t57164 = t2339 * t15093;
    let t57167 = t6602 * t4534;
    let t81168 = t2168 * t3961;
    let t82257 = t1322 * t220;
    let t82650 = t6211 * t1322;
    let t83423 = t6581 * t1591;
    let t83707 = t2326 * t4497;
    let t109134 = 3.0 * t31861;
    let t109135 = 12.0 * t31863;
    let t109136 = 6.0 * t31865;
    (t57164, t57167, t81168, t82257, t82650, t83423, t83707, t109134, t109135, t109136)
}
