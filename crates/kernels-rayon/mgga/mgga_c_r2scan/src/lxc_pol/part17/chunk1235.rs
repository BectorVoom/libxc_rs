//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1235/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1235(t39977: f64, t39982: f64, t41634: f64, t41635: f64, t41636: f64, t41637: f64, t41638: f64, t41642: f64, t41644: f64, t43447: f64, t43451: f64, t43454: f64) -> f64 {
    let t44396 = 0.27944763721877274748e0_f64 * t43447 - 0.46574606203128791246e-1_f64 * t43451 + t41634 + t41635 + 0.12805040077930161442e0_f64 * t43454 + t41636 + t41637 + t41638 - 0.85366933852867742947e0_f64 * t39977 - t41642 - 0.92461031893912198008e0_f64 * t39982 + t41644;
    t44396
}
