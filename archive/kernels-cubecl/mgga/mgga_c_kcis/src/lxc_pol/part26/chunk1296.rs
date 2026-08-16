//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1296/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1296<F: Float>(t23096: F, t6207: F, t99429: F, t17980: F, t28814: F, t99002: F, t29590: F, t4425: F, t7978: F, t29369: F, t4142: F, t1464: F, t29353: F, t94216: F) -> (F, F, F, F, F) {
    let t102166 = t99429 * t6207 * t23096;
    let t102170 = t99002 * t17980 * t28814;
    let t102174 = t7978 * t4425 * t29590;
    let t102180 = t4142 * t29369;
    let t102183 = t1464 * t94216 * t29353;
    (t102166, t102170, t102174, t102180, t102183)
}
