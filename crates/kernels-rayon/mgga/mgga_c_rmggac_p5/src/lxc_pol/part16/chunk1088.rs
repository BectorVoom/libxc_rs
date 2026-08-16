//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1088/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1088(t1587: f64, t2471: f64, t2868: f64, t35327: f64, t39694: f64, t39789: f64, t39792: f64, t39804: f64, t43100: f64, t43107: f64, t43108: f64, t43135: f64, t43138: f64, t43139: f64, t43141: f64, t45781: f64, t45788: f64, t45794: f64, t739: f64, t9383: f64) -> (f64, f64) {
    let t48638 = t2471 * t1587;
    let t48641 = t43100 - 0.15323255961587222184e-3_f64 * t45781 - 0.11974241701863808564e0_f64 * t2868 * t9383 + 0.43639458646792546768e0_f64 * t39694 + t43107 - t43108 - 0.5107751987195740728e-4_f64 * t45788 - 0.10909864661698136692e0_f64 * t45794 - 0.66211599834018861287e-4_f64 * t35327 - t43135 - 0.60975299583150056624e-3_f64 * t39789 - 0.78064147182743091554e-3_f64 * t39792 - t43138 - t43139 - 0.60975299583150056624e-3_f64 * t39804 + t43141 - 0.11974241701863808564e0_f64 * t739 * t48638;
    (t48638, t48641)
}
