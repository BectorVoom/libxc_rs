//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 853/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk853<F: Float>(t6597: F, t942: F, t2422: F, t2430: F, t2454: F, t411: F, t415: F, t6536: F, t6548: F, t6552: F, t938: F, t952: F, t1306: F, t135: F, t2457: F, t2464: F, t273: F, t6243: F, t6245: F, t6319: F, t6322: F, t6329: F, t6333: F, t6358: F, t6359: F, t6362: F, t6498: F, t6500: F, t6504: F, t955: F, t957: F) -> (F, F, F) {
    let t6598 = t942 * t6597;
    let t6601 = 0.65854491829355115987e0 * t6536 * t415 - 0.19756347548806534796e1 * t2422 * t952 + 0.39512695097613069591e1 * t938 * t2430 - 0.19756347548806534796e1 * t938 * t2454 - 0.39512695097613069591e1 * t411 * t6548 + 0.39512695097613069591e1 * t411 * t6552 - 0.65854491829355115987e0 * t411 * t6598;
    let t6605 = -3.0 * t1306 * t2457 * t2464 * t955 + 2.0 * t135 * t273 * t6359 * t6362 + t135 * t273 * t6601 * t957 - t6243 - t6245 - t6319 + t6322 - t6329 + t6333 + t6358 - t6498 + t6500 - t6504;
    (t6598, t6601, t6605)
}
