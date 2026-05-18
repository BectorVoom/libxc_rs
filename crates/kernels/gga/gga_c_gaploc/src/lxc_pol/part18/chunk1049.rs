//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1049/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1049<F: Float>(t10531: F, t1433: F, t539: F, t599: F, t4786: F, t6715: F, t1410: F, t6295: F, t900: F, t1339: F, t20013: F, t1415: F, t6834: F) -> (F, F, F, F, F, F, F) {
    let t20796 = t1433 * t10531;
    let t20800 = t539 * t599;
    let t20827 = t4786 * t6715;
    let t20843 = t1410 * t599;
    let t20887 = t900 * t6295;
    let t20896 = t1339 * t20013;
    let t20900 = t1415 * t6834;
    (t20796, t20800, t20827, t20843, t20887, t20896, t20900)
}
