//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1213/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1213<F: Float>(t15260: F, t15499: F, t15528: F, t15577: F, t1268: F, t11178: F, t1240: F, t13327: F, t13332: F, t13337: F, t13340: F, t13344: F, t13348: F, t1857: F, t9557: F, t9559: F, t9563: F) -> F {
    let t15579 = t15260 + t15499 + t15528 + t15577;
    let t15580 = t15579 * t1268;
    let t15585 = F::cast_from(0.17024129629629629629e-1_f64) * t13327 - F::cast_from(0.15476481481481481481e-2_f64) * t13332 - F::cast_from(0.23214722222222222222e-2_f64) * t13337 - F::cast_from(0.77382407407407407406e-3_f64) * t13340 + F::cast_from(0.61905925925925925926e-2_f64) * t13344 + F::cast_from(0.11349419753086419753e-1_f64) * t13348 - F::cast_from(0.15476481481481481481e-2_f64) * t9557 - F::cast_from(0.41270617283950617284e-2_f64) * t9559 - F::cast_from(0.51588271604938271604e-3_f64) * t9563 - F::new(0.66725e-1) * t1240 * t15580 - F::new(0.66725e-1) * t11178 * t1857;
    t15585
}
