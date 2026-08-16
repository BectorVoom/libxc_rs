//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 314/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk314(t1910: f64, t196: f64, t1019: f64, t1027: f64, t1044: f64, t1050: f64, t1087: f64, t1094: f64, t1104: f64, t1112: f64, t1133: f64, t1140: f64, t1143: f64, t1839: f64, t1843: f64, t1844: f64, t500: f64) -> (f64, f64) {
    let t1911 = t196 * t1910;
    let t1914 = -t1019 - t1027 - t1044 - t1050 + t1133 + 0.186546e0_f64 * t1143 * t1839 - t1094 + t1104 + t1112 - t1087 + t1140 + t1843 + 0.31091e-1_f64 * t1911 * t500 - t1844;
    (t1911, t1914)
}
