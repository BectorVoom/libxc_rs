//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1404/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1404<F: Float>(t17259: F, t17267: F, t17274: F, t17276: F, t2093: F, t4413: F, t1591: F, t6136: F, t12581: F, t1617: F, t17000: F, t17007: F, t17012: F, t17257: F, t17263: F, t17272: F, t17281: F, t17290: F, t17294: F, t17299: F, t17301: F, t2110: F, t4315: F, t4469: F, t6193: F) -> F {
    let t18244 = F::cast_from(0.23214722222222222222e-2_f64) * t17259;
    let t18246 = F::cast_from(0.25794135802469135802e-2_f64) * t17267;
    let t18248 = F::cast_from(0.30952962962962962962e-2_f64) * t17274;
    let t18249 = F::cast_from(0.10317654320987654321e-2_f64) * t17276;
    let t18253 = t2093 * t4413;
    let t18256 = t6136 * t1591;
    let t18263 = -F::cast_from(0.66725e-1_f64) * t6193 * t4469 - F::cast_from(0.17411041666666666666e-2_f64) * t17000 - F::cast_from(0.23214722222222222222e-2_f64) * t17007 - F::cast_from(0.11607361111111111111e-1_f64) * t17012 + F::cast_from(0.17411041666666666666e-2_f64) * t17257 + t18244 + F::cast_from(0.61905925925925925924e-2_f64) * t17263 + t18246 + F::cast_from(0.77382407407407407407e-3_f64) * t17272 - t18248 + t18249 - F::cast_from(0.11607361111111111111e-2_f64) * t17281 - F::cast_from(0.66725e-1_f64) * t12581 * t2110 + F::cast_from(0.890445125e-2_f64) * t18253 * t4315 - F::cast_from(0.13345e0_f64) * t18256 * t1617 + F::cast_from(0.15476481481481481481e-2_f64) * t17290 - F::cast_from(0.41270617283950617282e-2_f64) * t17294 + F::cast_from(0.15476481481481481481e-2_f64) * t17299 - F::cast_from(0.23214722222222222222e-2_f64) * t17301;
    t18263
}
