//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1265/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1265<F: Float>(t1993: F, t24827: F, t5444: F, t9155: F, t2455: F, t397: F, t2023: F, t9226: F, t1785: F, t7718: F, t220: F, t2464: F, t31861: F, t31863: F, t31865: F, t31875: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t73230 = t24827 * t1993;
    let t74221 = t9155 * t5444;
    let t74846 = t2455 * t397;
    let t74995 = t9226 * t2023;
    let t93749 = t7718 * t1785;
    let t93968 = t2464 * t220;
    let t109134 = 3.0 * t31861;
    let t109135 = 12.0 * t31863;
    let t109136 = 6.0 * t31865;
    let t109141 = 12.0 * t31875;
    (t73230, t74221, t74846, t74995, t93749, t93968, t109134, t109135, t109136, t109141)
}
