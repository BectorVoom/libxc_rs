//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 954/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk954<F: Float>(t1391: F, t3820: F, t3824: F, t443: F, t1346: F, t3832: F, t12830: F, t425: F, t1354: F, t3278: F, t1364: F, t1350: F, t3283: F) -> (F, F, F, F, F, F) {
    let t14116 = t1391 * t3820;
    let t14118 = t443 * t3824;
    let t14120 = t1346 * t3832;
    let t14122 = t425 * t12830;
    let t14125 = t1354 * t3278;
    let t14126 = t14125 * t1364;
    let t14129 = t1350 * t3283;
    (t14116, t14118, t14120, t14122, t14126, t14129)
}
