//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 957/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk957<F: Float>(t2199: F, t9194: F, t1341: F, t2239: F, t2200: F, t3373: F, t6919: F, t2247: F, t3369: F, t808: F, t2245: F, t1340: F, t6862: F, t6859: F, t2284: F, t6876: F, t6903: F, t9173: F, t9177: F, t9180: F, t9184: F, t9189: F, t9191: F, t9193: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9196 = 4.0 * t2199 * t9194;
    let t9197 = t1341 * t2239;
    let t9199 = 2.0 * t2199 * t9197;
    let t9200 = t3373 * t2200;
    let t9202 = 0.96491876992155210402e2 * t6919 * t9200;
    let t9203 = t3369 * t2247;
    let t9204 = t9203 * t808;
    let t9206 = 0.32163958997385070134e2 * t2245 * t9204;
    let t9207 = t3373 * t2239;
    let t9209 = 0.16081979498692535067e2 * t2245 * t9207;
    let t9210 = t1340 * t6862;
    let t9211 = t9210 * t2200;
    let t9213 = 0.51726012919273400301e3 * t6859 * t9211;
    let t9214 = -0.19298375398431042081e3 * t6876 * t9173 + 0.64327917994770140268e2 * t2284 * t9177 + 0.32163958997385070134e2 * t2284 * t9180 + 0.2069040516770936012e4 * t6903 * t9184 - t9189 + t9191 - t9193 + t9196 + t9199 + t9202 - t9206 - t9209 - t9213;
    (t9196, t9197, t9199, t9200, t9202, t9204, t9206, t9207, t9209, t9211, t9213, t9214)
}
