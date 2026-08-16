//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 955/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk955(t10405: f64, t10408: f64, t10411: f64, t10448: f64, t10451: f64, t10454: f64, t10478: f64, t1413: f64, t1449: f64, t3337: f64, t4218: f64, t430: f64, t453: f64, t4772: f64, t4828: f64, t8599: f64, t995: f64) -> f64 {
    let t10481 = 0.496875e-1_f64 * t4218 * t3337 - 0.99375e-1_f64 * t8599 * t995 + 0.298125e0_f64 * t4772 * t10405 - 0.99375e-1_f64 * t1413 * t10408 - 0.99375e-1_f64 * t1413 * t10411 + 0.165625e-1_f64 * t430 * t10448 - 0.19875e0_f64 * t4828 * t10451 + 0.1490625e0_f64 * t1449 * t10454 - 0.165625e-1_f64 * t453 * t10478;
    t10481
}
