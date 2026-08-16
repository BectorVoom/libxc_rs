//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2685/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2685(t1307: f64, t16018: f64, t16084: f64, t213: f64, t221: f64, t3719: f64, t40423: f64, t40425: f64, t40429: f64, t40431: f64, t5195: f64, t54663: f64, t54668: f64, t54671: f64, t54673: f64, t54676: f64) -> f64 {
    let t54687 = 0.38888888888888888888e-2_f64 * t40423 - 0.15833333333333333332e-1_f64 * t40425 + 0.83333333333333333332e-3_f64 * t40429 - 0.19999999999999999999e-1_f64 * t54663 + t54668 + 0.46666666666666666664e-1_f64 * t40431 + 0.13999999999999999999e0_f64 * t54671 - 0.34999999999999999998e-1_f64 * t54673 + 0.47499999999999999998e-1_f64 * t54676 + 0.14999999999999999999e-1_f64 * t5195 * t221 * t213 * t16018 * t1307 + 0.14999999999999999999e-1_f64 * t5195 * t221 * t16084 * t3719;
    t54687
}
