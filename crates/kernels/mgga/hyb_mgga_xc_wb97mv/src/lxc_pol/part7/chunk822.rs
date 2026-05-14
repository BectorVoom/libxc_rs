//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 822/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk822<F: Float>(t6260: F, t1937: F, t81: F, t1922: F, t621: F, t72: F, t126: F, t3003: F, t19: F, t2004: F, t546: F, t1975: F, t550: F, t2003: F, t668: F, t1836: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6261 = 1.0 / t6260;
    let t6265 = t81 * t1937;
    let t6285 = 1.0 / t1922 / t621;
    let t6296 = 1.0 / t6260 / t72;
    let t6381 = t3003 * t126;
    let t6383 = 5.0 / 288.0 * t19 * t6381;
    let t6384 = t546 * t2004;
    let t6386 = t1975 * t550;
    let t6388 = t2003 * t668;
    let t6389 = t19 * t6388;
    let t6391 = t546 * t1836;
    (t6261, t6265, t6285, t6296, t6381, t6383, t6384, t6386, t6388, t6389, t6391)
}
