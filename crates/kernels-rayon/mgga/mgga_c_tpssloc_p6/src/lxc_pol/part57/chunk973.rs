//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 973/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk973(t31193: f64, t6347: f64, t6637: f64, t6888: f64, t120492: f64, t1799: f64, t22685: f64, t6330: f64, t120490: f64, t1992: f64, t550: f64, t6976: f64, t97189: f64) -> (f64, f64, f64, f64, f64) {
    let t127371 = 0.3289868133696452873e-1_f64 * t6888 * t6637 * t31193 * t6347;
    let t127375 = 0.6579736267392905746e-1_f64 * t6888 * t6637 * t120492 * t1799;
    let t127381 = 0.9869604401089358619e-1_f64 * t22685 * t6637 * t31193 * t6330;
    let t127382 = 0.3289868133696452873e-1_f64 * t120490;
    let t127386 = 0.3289868133696452873e-1_f64 * t1992 * t6976 * t97189 * t550;
    (t127371, t127375, t127381, t127382, t127386)
}
