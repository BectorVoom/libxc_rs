//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 601/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk601<F: Float>(t4347: F, t548: F, t5610: F, t8080: F, t8084: F, t8087: F, t8091: F, t8095: F, t8165: F, t8173: F, t8178: F, t8182: F, t8289: F, t8396: F) -> F {
    let t8431 = -F::cast_from(0.23214722222222222222e-2_f64) * t8080 - F::cast_from(0.38691203703703703703e-3_f64) * t8084 + F::cast_from(0.23214722222222222222e-2_f64) * t8087 + F::cast_from(0.11607361111111111111e-2_f64) * t8091 + F::cast_from(0.19345601851851851852e-2_f64) * t8095 + F::cast_from(0.17411041666666666666e-2_f64) * t8165 + F::cast_from(0.15476481481481481481e-2_f64) * t5610 + t8396 * t548 + F::cast_from(0.74498e-1_f64) * t4347 * t8289 - F::cast_from(0.23214722222222222222e-2_f64) * t8173 + F::cast_from(0.15476481481481481481e-2_f64) * t8178 - F::cast_from(0.23214722222222222222e-2_f64) * t8182;
    t8431
}
