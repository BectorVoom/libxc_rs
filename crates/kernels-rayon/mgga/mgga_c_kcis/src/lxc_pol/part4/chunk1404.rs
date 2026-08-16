//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1404/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1404(t17259: f64, t17267: f64, t17274: f64, t17276: f64, t2093: f64, t4413: f64, t1591: f64, t6136: f64, t12581: f64, t1617: f64, t17000: f64, t17007: f64, t17012: f64, t17257: f64, t17263: f64, t17272: f64, t17281: f64, t17290: f64, t17294: f64, t17299: f64, t17301: f64, t2110: f64, t4315: f64, t4469: f64, t6193: f64) -> f64 {
    let t18244 = 0.23214722222222222222e-2_f64 * t17259;
    let t18246 = 0.25794135802469135802e-2_f64 * t17267;
    let t18248 = 0.30952962962962962962e-2_f64 * t17274;
    let t18249 = 0.10317654320987654321e-2_f64 * t17276;
    let t18253 = t2093 * t4413;
    let t18256 = t6136 * t1591;
    let t18263 = -0.66725e-1_f64 * t6193 * t4469 - 0.17411041666666666666e-2_f64 * t17000 - 0.23214722222222222222e-2_f64 * t17007 - 0.11607361111111111111e-1_f64 * t17012 + 0.17411041666666666666e-2_f64 * t17257 + t18244 + 0.61905925925925925924e-2_f64 * t17263 + t18246 + 0.77382407407407407407e-3_f64 * t17272 - t18248 + t18249 - 0.11607361111111111111e-2_f64 * t17281 - 0.66725e-1_f64 * t12581 * t2110 + 0.890445125e-2_f64 * t18253 * t4315 - 0.13345e0_f64 * t18256 * t1617 + 0.15476481481481481481e-2_f64 * t17290 - 0.41270617283950617282e-2_f64 * t17294 + 0.15476481481481481481e-2_f64 * t17299 - 0.23214722222222222222e-2_f64 * t17301;
    t18263
}
