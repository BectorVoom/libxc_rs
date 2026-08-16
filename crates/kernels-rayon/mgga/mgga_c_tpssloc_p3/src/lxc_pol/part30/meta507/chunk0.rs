//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1826/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1826(t25650: f64, t25651: f64, t1615: f64, t3128: f64, t1022: f64, t23678: f64, t1015: f64, t1011: f64, t360: f64, t1941: f64, t4616: f64, t23474: f64, t23480: f64, t23483: f64, t23500: f64, t23564: f64, t25639: f64, t25642: f64, t25645: f64, t378: f64, t4585: f64, t4609: f64, t6717: f64, t6747: f64, t6765: f64, t7583: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25652 = t25650 * t25651;
    let t25653 = t3128 * t1615;
    let t25654 = t23678 * t1022;
    let t25655 = t25653 * t25654;
    let t25658 = t1015 * t1615;
    let t25659 = t1011 * t1022;
    let t25660 = t25659 * t360;
    let t25661 = t25658 * t25660;
    let t25664 = t4616 * t1941;
    let t25672 = 0.10093189023535097714e-3_f64 * t23474 - 0.10093189023535097714e-3_f64 * t23480 + t6717 * t4609 / 288.0_f64 - 0.10093189023535097714e-3_f64 * t25639 + 0.10093189023535097714e-3_f64 * t25642 - 0.10093189023535097714e-3_f64 * t25645 * t6747 - 0.10093189023535097714e-3_f64 * t23564 * t7583 + 0.20186378047070195428e-3_f64 * t25652 * t25655 - 0.10093189023535097714e-3_f64 * t25652 * t25661 + t25664 * t378 / 1536.0_f64 + t23500 / 2304.0_f64 - 0.80745512188280781712e-3_f64 * t23483 * t7583 - t6765 * t4585 / 1152.0_f64;
    (t25652, t25653, t25654, t25655, t25658, t25660, t25661, t25664, t25672)
}
