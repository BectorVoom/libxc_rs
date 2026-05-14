//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 246/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk246<F: Float>(t1019: F, t1021: F, t1027: F, t1030: F, t1032: F, t1036: F, t1044: F, t1047: F, t1094: F, t1133: F, t975: F, t1011: F, t1014: F, t1017: F, t1050: F, t1087: F, t1104: F, t1112: F, t1136: F, t1140: F, t1142: F, t948: F, t982: F) -> (F, F) {
    let t1147 = -t1044 - t1047 - t975 + t1133 + t1036 + t1030 - t1032 - t1027 + t1021 + t1019 - t1094;
    let t1148 = t1011 + t1014 - t1017 - t1050 + t1142 + t1112 + t1104 + t948 - t1136 + t982 - t1087 + t1140;
    (t1147, t1148)
}
