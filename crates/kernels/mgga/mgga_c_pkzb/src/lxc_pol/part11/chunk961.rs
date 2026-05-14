//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 961/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk961<F: Float>(t11532: F, t942: F, t11484: F, t11494: F, t11497: F, t1246: F, t1256: F, t3904: F, t3910: F, t3929: F, t411: F, t415: F, t11143: F, t11159: F, t11231: F, t11236: F, t11238: F, t11316: F, t11318: F, t11321: F, t11325: F, t11329: F, t11355: F, t11363: F, t135: F, t273: F, t957: F) -> (F, F, F) {
    let t11533 = t942 * t11532;
    let t11536 = 0.65854491829355115987e0 * t11484 * t415 - 0.19756347548806534796e1 * t3904 * t1256 + 0.39512695097613069591e1 * t1246 * t3910 - 0.19756347548806534796e1 * t1246 * t3929 - 0.39512695097613069591e1 * t411 * t11494 + 0.39512695097613069591e1 * t411 * t11497 - 0.65854491829355115987e0 * t411 * t11533;
    let t11540 = t11536 * t135 * t273 * t957 + t11143 - t11159 - t11231 + t11236 + t11238 + t11316 + t11318 - t11321 - t11325 + t11329 - t11355 + t11363;
    (t11533, t11536, t11540)
}
