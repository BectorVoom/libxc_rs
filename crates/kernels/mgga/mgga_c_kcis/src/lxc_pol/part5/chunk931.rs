//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 931/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk931<F: Float>(t3255: F, t4608: F, t1071: F, t1114: F, t4634: F, t4597: F, t1035: F, t3293: F, t1727: F, t934: F, t313: F, t4600: F, t4639: F, t4644: F, t1670: F, t4572: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t14127 = 0.19711289e-2 * t3255 * t4608;
    let t14128 = t1114 * t1071;
    let t14137 = t3255 * t4634;
    let t14168 = 0.13140859333333333333e-2 * t3255 * t4597;
    let t14170 = t3293 * t1035;
    let t14171 = t1727 * t934;
    let t14196 = t4600 * t313;
    let t14202 = 0.19711289e-2 * t3255 * t4639;
    let t14204 = 0.26281718666666666666e-2 * t3255 * t4644;
    let t14215 = t1035 * t1670;
    let t14232 = 0.13140859333333333334e-2 * t3255 * t4572;
    (t14127, t14128, t14137, t14168, t14170, t14171, t14196, t14202, t14204, t14215, t14232)
}
