//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1324/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1324<F: Float>(t1937: F, t3559: F, t1096: F, t17388: F, t17601: F, t17638: F, t1916: F, t1917: F, t1932: F, t1933: F, t1938: F, t1941: F, t1956: F, t1977: F, t21229: F, t25956: F, t25971: F, t2796: F, t2816: F, t3565: F, t3578: F, t3581: F, t3592: F, t3605: F, t5830: F, t5838: F, t5871: F, t7293: F, t7296: F, t7315: F, t7324: F, t7407: F, t7408: F, t7447: F, t9401: F, t9429: F, t9518: F) -> (F,) {
    let t26224 = t3559 * t1937;
    let t26263 = -0.12304822629859687989e5 * t17638 * t9401 * t1956 - t25956 + 1.0 * t9518 * t1933 + 0.32163958997385070134e2 * t26224 * t1941 + 2.0 * t21229 * t1096 + 4.0 * t7447 * t2816 + 2.0 * t2796 * t7408 - 2.0 * t17388 * t3565 + t25971 + 6.0 * t1938 * t3578 * t1917 + 0.70178683471615754484e1 * t7315 * t7296 + 0.35089341735807877242e1 * t1977 * t3605 * t1956 + 12.0 * t7324 * t7293 - 24.0 * t5830 * t3565 * t1917 - 0.14035736694323150897e2 * t5838 * t3592 * t1956 + 0.11579025239058625248e4 * t5871 * t3581 * t1917 - 4.0 * t1916 * t1096 * t7407 - 0.19298375398431042081e3 * t5830 * t3581 * t1932 - 0.24828486201251232145e5 * t17601 * t9429 * t1917;
    (t26263,)
}
