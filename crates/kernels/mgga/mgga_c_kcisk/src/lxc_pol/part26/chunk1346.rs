//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1346/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1346<F: Float>(t32058: F, t415: F, t7907: F, t1339: F, t32203: F, t34809: F, t1308: F, t388: F, t80222: F, t33451: F, t33460: F, t114021: F, t26760: F, t3482: F, t109883: F, t26809: F) -> (F, F, F, F, F, F) {
    let t119675 = t415 * t32058 * t7907;
    let t119685 = t1339 * t32203 * t34809;
    let t119688 = t80222 * t388 * t1308;
    let t119693 = t33460 * t33451;
    let t119698 = t3482 * t114021 * t26760;
    let t119701 = t3482 * t109883 * t26809;
    (t119675, t119685, t119688, t119693, t119698, t119701)
}
