//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 251/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk251(t1019: f64, t1021: f64, t1027: f64, t1030: f64, t1032: f64, t1036: f64, t1044: f64, t1047: f64, t1094: f64, t1133: f64, t975: f64, t1011: f64, t1014: f64, t1017: f64, t1050: f64, t1087: f64, t1104: f64, t1112: f64, t1136: f64, t1140: f64, t1142: f64, t948: f64, t982: f64) -> (f64, f64) {
    let t1147 = -t1044 - t1047 - t975 + t1133 + t1036 + t1030 - t1032 - t1027 + t1021 + t1019 - t1094;
    let t1148 = t1011 + t1014 - t1017 - t1050 + t1142 + t1112 + t1104 + t948 - t1136 + t982 - t1087 + t1140;
    (t1147, t1148)
}
