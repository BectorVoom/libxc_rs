//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1259/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1259<F: Float>(t11002: F, t2211: F, t2224: F, t238: F, t4180: F, t3326: F, t242: F, t3346: F, t790: F, t8951: F, t8954: F, t2205: F, t26298: F, t26301: F, t26304: F, t30747: F, t30750: F, t30778: F) -> (F, F, F, F, F, F, F, F) {
    let t30861 = t11002 * t2211;
    let t30867 = t238 * t2224 * t4180;
    let t30869 = t3326 * t3326;
    let t30871 = t238 * t242 * t30869;
    let t30876 = t790 * t3346;
    let t30877 = t8951 * t30876;
    let t30879 = t8954 * t30876;
    let t30881 = t3346 * t3346;
    let t30882 = t2205 * t30881;
    let t30884 = -0.76790625e-1 * t30861 - 0.1860237037037037037e1 * t26298 + 0.15944888888888888889e1 * t26301 - 0.59793333333333333334e0 * t26304 + 0.27385555555555555555e0 * t30867 + 0.49294e0 * t30871 + 0.39862222222222222223e0 * t30747 - 0.59793333333333333334e0 * t30750 + 0.8969e0 * t30778 + 0.5696775e1 * t30877 - 0.3071625e0 * t30879 - 0.1898925e1 * t30882;
    (t30861, t30867, t30871, t30877, t30879, t30881, t30882, t30884)
}
