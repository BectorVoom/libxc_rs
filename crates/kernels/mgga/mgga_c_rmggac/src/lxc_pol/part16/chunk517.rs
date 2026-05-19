//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 517/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk517<F: Float>(t1704: F, t305: F, t326: F, t344: F, t3826: F, t3839: F, t5840: F, t6376: F, t6403: F, t6412: F, t6415: F, t6418: F, t6421: F, t6425: F, t6434: F, t6441: F, t6444: F, t6449: F, t793: F, t797: F, t851: F, t854: F, t861: F) -> F {
    let t6462 = F::cast_from(0.53104616420242325356e-2_f64) * t3839 * t6425 - F::cast_from(0.79656924630363488034e-2_f64) * t3826 * t6403 - F::cast_from(0.19957069503106347607e-1_f64) * t326 * t6376 + F::cast_from(0.26552308210121162678e-3_f64) * t344 * t5840 - F::cast_from(0.31862769852145395214e-2_f64) * t854 * t6434 + F::cast_from(0.13276154105060581339e-2_f64) * t851 * t6415 - F::cast_from(0.15931384926072697607e-2_f64) * t854 * t6418 - F::cast_from(0.59871208509319042821e-1_f64) * t797 * t6441 + F::cast_from(0.39914139006212695214e-1_f64) * t6444 * t1704 + F::cast_from(0.79828278012425390428e-1_f64) * t793 * t6412 - F::cast_from(0.11974241701863808564e0_f64) * t797 * t6449 - F::cast_from(0.11974241701863808564e0_f64) * t797 * t6434 - F::cast_from(0.15931384926072697607e-2_f64) * t854 * t6441 + F::cast_from(0.18586615747084813875e-2_f64) * t861 * t6421 - F::cast_from(0.31862769852145395214e-2_f64) * t854 * t6449 + F::cast_from(0.19957069503106347607e-1_f64) * t305 * t5840;
    t6462
}
