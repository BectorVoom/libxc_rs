//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 669/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk669<F: Float>(t4059: F, t1444: F, t740: F, t833: F, t1437: F, t3805: F, t1330: F, t3797: F, t111: F, t1404: F, t1445: F, t2645: F, t4047: F, t4050: F, t4053: F, t4054: F, t4055: F) -> (F, F, F, F, F, F) {
    let t4060 = 0.15538616723388920628e-3 * t4059;
    let t4061 = t740 * t1444;
    let t4062 = t4061 * t833;
    let t4066 = t1437 * t3805;
    let t4069 = t1330 * t3797;
    let t4072 = t4047 - t4050 - t4053 - t4054 - 0.23911438650126355246e-1 * t4055 + 0.11955719325063177623e-1 * t1404 * t2645 + t4060 + 0.20718155631185227504e-3 * t4062 - 0.5179538907796306876e-4 * t1445 * t2645 + 0.7925e-3 * t111 * t4066 - 0.52833333333333333333e-3 * t111 * t4069;
    (t4060, t4061, t4062, t4066, t4069, t4072)
}
