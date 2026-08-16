//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1864/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1864(t25132: f64, t81882: f64, t6604: f64, t81968: f64, t13184: f64, t841: f64, t23083: f64, t25123: f64, t13191: f64, t25119: f64, t1878: f64, t81982: f64) -> (f64, f64, f64, f64, f64) {
    let t87405 = t81882 * t25132;
    let t87407 = t81968 * t6604;
    let t87409 = t87407 * t841 * t13184;
    let t87411 = t23083 * t25123;
    let t87418 = t25119 * t841 * t13191;
    let t87420 = t1878 * t81982;
    (t87405, t87409, t87411, t87418, t87420)
}
