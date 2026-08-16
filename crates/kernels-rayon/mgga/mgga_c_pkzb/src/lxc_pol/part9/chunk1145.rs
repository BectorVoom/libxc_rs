//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1145/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1145(t24: f64, t2569: f64, t500: f64, t1165: f64, t19660: f64, t19663: f64, t19669: f64, t19672: f64, t3019: f64, t3022: f64, t333: f64, t507: f64, t5107: f64, t5113: f64, t7932: f64, t7935: f64, t7940: f64, t82: f64, zeta_threshold: f64) -> f64 {
    let t90 = t24 <= zeta_threshold;
    let t19863 = 16.0_f64 * t2569 * t500;
    let t19865 = piecewise3(t90, 0.0_f64, -56.0_f64 / 81.0_f64 * t7932 * t5107 - 16.0_f64 / 9.0_f64 * t7935 * t19660 + 8.0_f64 / 9.0_f64 * t3019 * t19663 + 4.0_f64 / 3.0_f64 * t7940 * t507 - 4.0_f64 * t3022 * t19669 + 4.0_f64 / 3.0_f64 * t3022 * t19672 - 2.0_f64 / 9.0_f64 * t1165 * t5113 + 8.0_f64 * t333 * t82 - t19863);
    t19865
}
