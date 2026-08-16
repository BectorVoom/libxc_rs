//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2047/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2047(t6604: f64, t81968: f64, t23083: f64, t25123: f64, t1878: f64, t81982: f64, t25120: f64, t81962: f64, t7500: f64, t81911: f64, t22690: f64, t23122: f64, t4119: f64, t841: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87407 = t81968 * t6604;
    let t87411 = t23083 * t25123;
    let t87412 = 0.28260929265898273598e-2_f64 * t87411;
    let t87420 = t1878 * t81982;
    let t87425 = t81962 * t6604 * t25120;
    let t87426 = 0.11869590291677274911e0_f64 * t87425;
    let t87432 = t81911 * t7500;
    let t87443 = t23122 * t22690 * t841 * t4119;
    (t87407, t87412, t87420, t87426, t87432, t87443)
}
