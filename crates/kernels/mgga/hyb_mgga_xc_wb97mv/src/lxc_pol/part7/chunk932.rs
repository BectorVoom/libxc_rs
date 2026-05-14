//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 932/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk932<F: Float>(t2109: F, t3205: F, t151: F, t154: F, t157: F, t1246: F, t6507: F, t1249: F, t1251: F, t1253: F, t1255: F, t1257: F, t1259: F, t1261: F, t2053: F, t2081: F, t3182: F, t3208: F, t3213: F, t3238: F, t707: F) -> (F, F) {
    let t8722 = t2109 * t3205;
    let t8727 = t151 * t3205;
    let t8732 = t154 * t3205;
    let t8737 = t157 * t3205;
    let t8756 = t6507 * t1246;
    let t8759 = -t8722 * t707 / 0.37158912e10 - t3238 * t2081 / 0.74317824e10 + t8727 * t707 / 3.0 + t3182 * t2081 / 6.0 - t8732 * t707 / 24.0 - t3208 * t2081 / 48.0 + t8737 * t707 / 320.0 + t3213 * t2081 / 640.0 - 2.0 / 3.0 * t1249 * t2053 + t1251 * t2053 / 8.0 - t1253 * t2053 / 80.0 + t1255 * t2053 / 1152.0 - t1257 * t2053 / 21504.0 + t1259 * t2053 / 491520.0 - t1261 * t2053 / 13271040.0 + t8756 * t2053 / 412876800.0;
    (t8756, t8759)
}
