//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 362/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk362<F: Float>(t201: F, t109: F, t198: F, t212: F, t417: F, t1291: F, t1295: F, t1298: F, t193: F, t202: F, t210: F, tau1: F) -> (F, F, F, F, F, F, F) {
    let t1299 = t201 * t201;
    let t1300 = 1.0 / t1299;
    let t1304 = t109 * tau1;
    let t1305 = t198 * t212;
    let t1308 = t417 * tau1;
    let t1309 = t1308 * t198;
    let t1312 = -0.10666666666666666667e-1 * t193 * t1291 * t202 + 0.42666666666666666668e-4 * t1295 * t1298 * t1300 + 5.0 / 3.0 * t1304 * t1305 + 5.0 / 3.0 * t210 * t1309;
    (t1299, t1300, t1304, t1305, t1308, t1309, t1312)
}
