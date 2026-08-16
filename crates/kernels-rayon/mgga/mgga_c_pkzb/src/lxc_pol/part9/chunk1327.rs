//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1327/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1327(t1246: f64, t12845: f64, t158: f64, t23394: f64, t23398: f64, t23475: f64, t23535: f64, t2422: f64, t2428: f64, t2429: f64, t2453: f64, t2454: f64, t3247: f64, t3255: f64, t3278: f64, t411: f64, t415: f64, t6546: f64, t6552: f64, t8497: f64, t8501: f64, t8559: f64, t8560: f64, t938: f64, t942: f64, t951: f64) -> f64 {
    let t23542 = 0.79025390195226139182e1_f64 * t938 * t8501 + 0.39512695097613069591e1_f64 * t2422 * t3255 + 0.39512695097613069591e1_f64 * t411 * t2428 * t8559 * t951 + 0.65854491829355115987e0_f64 * t23394 * t158 * t415 - 0.11853808529283920877e2_f64 * t23398 * t12845 * t2453 - 0.19756347548806534796e1_f64 * t3247 * t2454 - 0.11853808529283920877e2_f64 * t411 * t6546 * t3278 * t2429 - 0.11853808529283920877e2_f64 * t938 * t8497 + 0.39512695097613069591e1_f64 * t1246 * t6552 - 0.65854491829355115987e0_f64 * t411 * t942 * (t23475 + t23535) - 0.19756347548806534796e1_f64 * t938 * t8560;
    t23542
}
