//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 650/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk650(t1268: f64, t3616: f64, t1267: f64, t426: f64, t1236: f64, t1239: f64, t1240: f64, t1269: f64, t2818: f64, t2823: f64, t2827: f64, t2832: f64, t2848: f64, t2853: f64, t2858: f64, t2862: f64, t3052: f64, t3172: f64, t3174: f64, t3180: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3617 = t3616 * t1268;
    let t3620 = t1267 * t1267;
    let t3621 = t426 * t426;
    let t3622 = 1.0_f64 / t3621;
    let t3623 = t3620 * t3622;
    let t3638 = t1236 * t1239;
    let t3641 = -0.66725e-1_f64 * t1240 * t3617 + 0.66725e-1_f64 * t1240 * t3623 - 0.23214722222222222222e-2_f64 * t2818 + 0.15476481481481481481e-2_f64 * t2823 + 0.23214722222222222222e-2_f64 * t2827 + 0.11607361111111111111e-2_f64 * t2832 + 0.19345601851851851852e-2_f64 * t2848 - 0.23214722222222222222e-2_f64 * t2853 - 0.61905925925925925925e-2_f64 * t2858 - 0.23214722222222222222e-2_f64 * t2862 + 0.23214722222222222222e-2_f64 * t3052 + 0.17411041666666666666e-2_f64 * t3172 + 0.15476481481481481481e-2_f64 * t3174 - 0.34822083333333333332e-2_f64 * t3180 - 0.13345e0_f64 * t3638 * t1269;
    (t3617, t3620, t3621, t3622, t3623, t3638, t3641)
}
