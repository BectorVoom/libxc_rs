//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 994/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk994(t121506: f64, t1484: f64, t6552: f64, t6637: f64, t114655: f64, t121501: f64, t126433: f64, t126437: f64, t126441: f64, t127959: f64, t127963: f64, t31394: f64, t33388: f64, t4166: f64, t5585: f64, t5612: f64, t5617: f64, t812: f64) -> f64 {
    let t127967 = t6552 * t6637 * t121506 * t1484;
    let t127979 = t126433 - t126437 + t126441 + 0.49348022005446793095e-1_f64 * t127959 + 0.3289868133696452873e-1_f64 * t127963 - 0.3289868133696452873e-1_f64 * t127967 + 0.16449340668482264365e-1_f64 * t121501 - t812 * t31394 * t5612 + 2.0_f64 * t812 * t114655 * t5585 - t812 * t31394 * t5617 - 2.0_f64 * t4166 * t33388;
    t127979
}
