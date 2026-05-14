//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1098/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1098<F: Float>(t24896: F, t24923: F, t24960: F, t24988: F, t25020: F, t25050: F, t25116: F, t25133: F, t2029: F, t18826: F, t18829: F, t18831: F, t18833: F, t1994: F, t24059: F, t24063: F, t24066: F, t24070: F, t24073: F, t24499: F, t5445: F) -> (F, F) {
    let t25136 = t24896 + t24923 + t24960 + t24988 + t25020 + t25050 + t25116 + t25133;
    let t25137 = t25136 * t2029;
    let t25147 = -0.193e0 * t1994 * t25137 + 0.148996e0 * t5445 * t24499 - 0.51588271604938271603e-3 * t24059 + t18826 - 0.25794135802469135802e-3 * t24063 - 0.41270617283950617283e-2 * t24066 + 0.77382407407407407407e-3 * t24070 - t18829 - t18831 + t18833 - 0.61905925925925925924e-2 * t24073;
    (t25136, t25147)
}
