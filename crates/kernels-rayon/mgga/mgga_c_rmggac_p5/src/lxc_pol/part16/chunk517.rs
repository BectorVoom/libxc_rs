//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 517/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk517(t1704: f64, t305: f64, t326: f64, t344: f64, t3826: f64, t3839: f64, t5840: f64, t6376: f64, t6403: f64, t6412: f64, t6415: f64, t6418: f64, t6421: f64, t6425: f64, t6434: f64, t6441: f64, t6444: f64, t6449: f64, t793: f64, t797: f64, t851: f64, t854: f64, t861: f64) -> f64 {
    let t6462 = 0.53104616420242325356e-2_f64 * t3839 * t6425 - 0.79656924630363488034e-2_f64 * t3826 * t6403 - 0.19957069503106347607e-1_f64 * t326 * t6376 + 0.26552308210121162678e-3_f64 * t344 * t5840 - 0.31862769852145395214e-2_f64 * t854 * t6434 + 0.13276154105060581339e-2_f64 * t851 * t6415 - 0.15931384926072697607e-2_f64 * t854 * t6418 - 0.59871208509319042821e-1_f64 * t797 * t6441 + 0.39914139006212695214e-1_f64 * t6444 * t1704 + 0.79828278012425390428e-1_f64 * t793 * t6412 - 0.11974241701863808564e0_f64 * t797 * t6449 - 0.11974241701863808564e0_f64 * t797 * t6434 - 0.15931384926072697607e-2_f64 * t854 * t6441 + 0.18586615747084813875e-2_f64 * t861 * t6421 - 0.31862769852145395214e-2_f64 * t854 * t6449 + 0.19957069503106347607e-1_f64 * t305 * t5840;
    t6462
}
