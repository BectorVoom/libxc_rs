//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 963/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk963<F: Float>(t1410: F, t599: F, t6295: F, t900: F, t1339: F, t20013: F, t1415: F, t6834: F, t1422: F, t161: F, t1353: F, t2486: F, t4624: F, t1428: F, t4398: F, t197: F, t2293: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20843 = t1410 * t599;
    let t20887 = t900 * t6295;
    let t20896 = t1339 * t20013;
    let t20900 = t1415 * t6834;
    let t20901 = t1422 * t161;
    let t20902 = t20901 * t1353;
    let t20954 = t4624 * t2486;
    let t20957 = t4398 * t1428;
    let t21004 = t197 * t2293;
    (t20843, t20887, t20896, t20900, t20901, t20902, t20954, t20957, t21004)
}
