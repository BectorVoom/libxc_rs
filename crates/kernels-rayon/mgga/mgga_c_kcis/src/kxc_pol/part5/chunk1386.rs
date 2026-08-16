//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1386/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1386(t2080: f64, t6097: f64, t1571: f64, t7463: f64, t12689: f64, t17797: f64, t17834: f64, t21172: f64, t21174: f64, t21176: f64, t21178: f64, t21180: f64, t21293: f64, t21295: f64, t22833: f64, t4331: f64, t4356: f64, t6080: f64, t6102: f64) -> f64 {
    let t22836 = t2080 * t6097;
    let t22839 = t7463 * t1571;
    let t22842 = -t21172 - t21174 - t21176 + t21178 - t21180 - t21293 - t21295 - 4.0_f64 * t17834 * t6080 + 0.64329366355741395948e2_f64 * t17797 * t6102 + 6.0_f64 * t4356 * t22833 - 4.0_f64 * t4331 * t22836 - 0.19298809906722418785e3_f64 * t12689 * t22839;
    t22842
}
