//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2113/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2113(t87233: f64, t25068: f64, t2703: f64, t81764: f64, t23127: f64, t4257: f64, t1512: f64, t81807: f64, t25146: f64, t2686: f64, t81824: f64, t81821: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t87234 = 0.13457585364713463618e-3_f64 * t87233;
    let t87235 = t25068 * t2703;
    let t87237 = 119.0_f64 / 864.0_f64 * t81764;
    let t87241 = t23127 * t4257;
    let t87243 = t81807 * t1512;
    let t87245 = t25146 * t2686;
    let t87247 = t81824 * t1512;
    let t87248 = 7.0_f64 / 1152.0_f64 * t87247;
    let t87249 = t81821 * t1512;
    (t87234, t87235, t87237, t87241, t87243, t87245, t87248, t87249)
}
