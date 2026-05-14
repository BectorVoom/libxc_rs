//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 883/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk883<F: Float>(t2320: F, t38621: F, t6557: F, t7778: F, t903: F, t26857: F, t8410: F, t6355: F, t8542: F, t2283: F, t38355: F, t8571: F, t8582: F, t40278: F, t8443: F, t1704: F, t352: F) -> (F, F, F, F, F, F, F, F) {
    let t47081 = t38621 * t2320;
    let t47100 = t903 * t7778 * t6557;
    let t47108 = t26857 * t8410;
    let t47110 = t6355 * t8542;
    let t47112 = t38355 * t2283;
    let t47114 = t8571 * t8582;
    let t47119 = t40278 * t8443;
    let t47124 = t1704 * t352;
    (t47081, t47100, t47108, t47110, t47112, t47114, t47119, t47124)
}
