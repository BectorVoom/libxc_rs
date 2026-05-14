//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1206/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1206<F: Float>(t1246: F, t12845: F, t158: F, t23394: F, t23398: F, t23475: F, t23535: F, t2422: F, t2428: F, t2429: F, t2453: F, t2454: F, t3247: F, t3255: F, t3278: F, t411: F, t415: F, t6546: F, t6552: F, t8497: F, t8501: F, t8559: F, t8560: F, t938: F, t942: F, t951: F) -> (F,) {
    let t23542 = 0.79025390195226139182e1 * t938 * t8501 + 0.39512695097613069591e1 * t2422 * t3255 + 0.39512695097613069591e1 * t411 * t2428 * t8559 * t951 + 0.65854491829355115987e0 * t23394 * t158 * t415 - 0.11853808529283920877e2 * t23398 * t12845 * t2453 - 0.19756347548806534796e1 * t3247 * t2454 - 0.11853808529283920877e2 * t411 * t6546 * t3278 * t2429 - 0.11853808529283920877e2 * t938 * t8497 + 0.39512695097613069591e1 * t1246 * t6552 - 0.65854491829355115987e0 * t411 * t942 * (t23475 + t23535) - 0.19756347548806534796e1 * t938 * t8560;
    (t23542,)
}
