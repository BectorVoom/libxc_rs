//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1025/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1025(t1249: f64, t6367: f64, t6366: f64, t2029: f64, t3199: f64, t3187: f64, t406: f64, t2376: f64, t3214: f64, t1238: f64, t2407: f64, t3195: f64, t6475: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8376 = t1249 * t6367;
    let t8377 = t6366 * t8376;
    let t8380 = t3199 * t2029;
    let t8381 = t8380 * t3187;
    let t8382 = t406 * t8381;
    let t8386 = 0.15244095330869239812e-2_f64 * t3214 * t2376;
    let t8389 = 0.30488190661738479624e-2_f64 * t1238 * t2407;
    let t8392 = t6475 * t3195;
    (t8376, t8377, t8380, t8381, t8382, t8386, t8389, t8392)
}
