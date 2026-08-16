//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1801/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1801(t81912: f64, t1878: f64, t81982: f64, t25120: f64, t6604: f64, t81962: f64, t7500: f64, t81911: f64, t81928: f64, t81934: f64, t81943: f64, t22690: f64, t23122: f64, t4119: f64, t841: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t87414 = 0.22608743412718618878e-1_f64 * t81912;
    let t87420 = t1878 * t81982;
    let t87425 = t81962 * t6604 * t25120;
    let t87432 = t81911 * t7500;
    let t87437 = 119.0_f64 / 3456.0_f64 * t81928;
    let t87438 = 0.13565246047631171327e0_f64 * t81934;
    let t87440 = 35.0_f64 / 108.0_f64 * t81943;
    let t87443 = t23122 * t22690 * t841 * t4119;
    (t87414, t87420, t87425, t87432, t87437, t87438, t87440, t87443)
}
