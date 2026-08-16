//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1033/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1033(t30663: f64, t6555: f64, t6552: f64, t6572: f64, t1880: f64, t30626: f64, t30630: f64, t30637: f64, t30640: f64, t30645: f64, t30647: f64, t30651: f64, t30655: f64, t30659: f64, t30662: f64, t6627: f64, t6632: f64, t855: f64) -> (f64, f64, f64) {
    let t30664 = t30663 * t6555;
    let t30666 = 0.3289868133696452873e-1_f64 * t6552 * t30664;
    let t30667 = t30663 * t6572;
    let t30669 = 0.16449340668482264365e-1_f64 * t1880 * t30667;
    let t30670 = 4.0_f64 * t30630 * t855 + 2.0_f64 * t30647 * t855 - 6.0_f64 * t30651 * t855 + 4.0_f64 * t6627 * t6632 + t30626 + t30637 - t30640 + t30645 - t30655 - t30659 + t30662 - t30666 - t30669;
    (t30664, t30667, t30670)
}
