//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1380/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1380(t14680: f64, t26871: f64, t3331: f64, t33862: f64, t8064: f64, t2189: f64, t46015: f64, t26868: f64, t5189: f64, t14668: f64, t26886: f64, t3330: f64, t3481: f64, t8081: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97507 = 4.0_f64 * t26871 * t14680;
    let t97510 = 24.0_f64 * t33862 * t8064 * t3331;
    let t97511 = t46015 * t2189;
    let t97513 = 2.0_f64 * t26868 * t5189;
    let t97517 = 2.0_f64 * t14668 * t26886;
    let t97521 = 2.0_f64 * t3330 * t8081 * t3481;
    (t97507, t97510, t97511, t97513, t97517, t97521)
}
