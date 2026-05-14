//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 230/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk230<F: Float>(t1128: F, t1129: F, t1004: F, t1097: F, t1101: F, t282: F, t939: F, t977: F, t288: F) -> (F, F, F, F) {
    let t1130 = t1128 * t1129;
    let t1136 = t1097 * t282 - 0.193e0 * t1101 * t1130 - 0.92858888888888888886e-2 * t939 + 0.69644166666666666665e-2 * t977 - 0.69644166666666666665e-2 * t1004;
    let t1138 = t288 * t288;
    let t1139 = 1.0 / t1138;
    (t1130, t1136, t1138, t1139)
}
