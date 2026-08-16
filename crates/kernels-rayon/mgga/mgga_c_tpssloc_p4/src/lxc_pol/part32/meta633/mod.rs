//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2046;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta633(t87363: f64, t242: f64, t812: f64, t81816: f64, t25064: f64, t81788: f64, t25135: f64, t838: f64, t2693: f64, t7503: f64, t25132: f64, t81882: f64, t6604: f64, t81968: f64, t23083: f64, t25123: f64, t1878: f64, t81982: f64, t25120: f64, t81962: f64, t7500: f64, t81911: f64, t22690: f64, t23122: f64, t4119: f64, t841: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87364, t87368, t87387, t87402, t87403, t87405) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2046(t87363, t242, t812, t81816, t25064, t81788, t25135, t838, t2693, t7503, t25132, t81882);
        let (t87407, t87412, t87420, t87426, t87432, t87443) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2047(t6604, t81968, t23083, t25123, t1878, t81982, t25120, t81962, t7500, t81911, t22690, t23122, t4119, t841);
    (t87364, t87368, t87387, t87402, t87403, t87405, t87407, t87412, t87420, t87426, t87432, t87443)
}
