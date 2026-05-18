//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 262/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk262<F: Float>(t198: F, t673: F, t1193: F, t209: F, t476: F, t446: F, t221: F, t1149: F, t205: F, t1156: F, t23: F, t1144: F) -> (F, F, F, F, F, F) {
    let t1194 = t673 * t198;
    let t1195 = t1193 * t1194;
    let t1196 = t476 * t209;
    let t1197 = t1196 * t446;
    let t1198 = t221 * t1197;
    let t1201 = t1149 * t205;
    let t1205 = t23 * t1156;
    let t1206 = t1205 * t1144;
    (t1194, t1195, t1196, t1198, t1201, t1206)
}
