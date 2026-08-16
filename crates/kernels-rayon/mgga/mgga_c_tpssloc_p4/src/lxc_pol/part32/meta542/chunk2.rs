//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1889/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1889(t1193: f64, t8020: f64, t1198: f64, t2134: f64, t24723: f64, t24729: f64, t24733: f64, t24741: f64, t27651: f64, t27655: f64, t27674: f64, t4950: f64, t4954: f64, t4980: f64, t4984: f64, t5046: f64, t7310: f64, t7316: f64, t7321: f64, t8028: f64, t8031: f64, t8035: f64) -> (f64, f64) {
    let t27677 = t8020 * t1193;
    let t27679 = -0.10093189023535097714e-3_f64 * t27651 + 0.10093189023535097714e-3_f64 * t24723 - 0.10093189023535097714e-3_f64 * t2134 * t27655 + 0.10093189023535097714e-3_f64 * t7316 * t8035 - t24741 * t4950 / 2304.0_f64 - t24741 * t4954 / 2304.0_f64 + t24729 * t4980 / 768.0_f64 - t24733 * t4984 / 1536.0_f64 - t7310 * t5046 / 288.0_f64 + 0.80745512188280781712e-3_f64 * t8028 * t7321 + 0.10093189023535097714e-3_f64 * t8031 * t7321 + t27674 * t1198 / 108.0_f64 - t27677 / 108.0_f64;
    (t27677, t27679)
}
