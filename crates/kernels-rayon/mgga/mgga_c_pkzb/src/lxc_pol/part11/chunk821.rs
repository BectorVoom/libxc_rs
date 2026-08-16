//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 821/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk821(t1020: f64, t637: f64, t6819: f64, t6821: f64, t6826: f64, t135: f64, t1535: f64, t192: f64, t2536: f64, t2537: f64, t2714: f64, t2718: f64, t3401: f64, t4867: f64, t4870: f64, t4876: f64, t4879: f64, t4886: f64, t5077: f64, t568: f64, t6758: f64, t6803: f64, t6810: f64, t6813: f64, t8716: f64, t8719: f64, t8720: f64, t8750: f64, t8751: f64) -> (f64, f64, f64, f64, f64) {
    let t8758 = t1020 * t637;
    let t8762 = 40.0_f64 * t6819;
    let t8763 = 24.0_f64 * t6821;
    let t8764 = 2.0_f64 * t6826;
    let t8768 = 6.0_f64 * t135 * t192 * t3401 * t568 - 6.0_f64 * t1535 * t2537 * t8758 - t2536 * t637 * t8751 + 12.0_f64 * t2714 * t2718 * t6758 + t4867 + t4870 - t4876 - t4879 + t4886 + t5077 - t6803 - t6810 - t6813 + t8716 - t8719 + t8720 + t8750 + t8762 - t8763 + t8764;
    (t8758, t8762, t8763, t8764, t8768)
}
