//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1435/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1435<F: Float>(t12485: F, t1749: F, t12428: F, t1737: F, t12247: F, t1719: F, t12226: F, t1261: F, t1715: F, t247: F, t44701: F, t1247: F, t1796: F, t42994: F) -> (F, F, F, F, F, F) {
    let t58262 = t1749 * t12485;
    let t58304 = t1737 * t12428;
    let t58342 = t1719 * t12247;
    let t58473 = t1719 * t12226;
    let t58777 = t1261 * t247 * t44701 * t1715;
    let t58824 = t1247 * t42994 * t1796;
    (t58262, t58304, t58342, t58473, t58777, t58824)
}
