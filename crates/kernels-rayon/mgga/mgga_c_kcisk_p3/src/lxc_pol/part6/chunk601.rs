//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 601/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk601(t4347: f64, t548: f64, t5610: f64, t8080: f64, t8084: f64, t8087: f64, t8091: f64, t8095: f64, t8165: f64, t8173: f64, t8178: f64, t8182: f64, t8289: f64, t8396: f64) -> f64 {
    let t8431 = -0.23214722222222222222e-2_f64 * t8080 - 0.38691203703703703703e-3_f64 * t8084 + 0.23214722222222222222e-2_f64 * t8087 + 0.11607361111111111111e-2_f64 * t8091 + 0.19345601851851851852e-2_f64 * t8095 + 0.17411041666666666666e-2_f64 * t8165 + 0.15476481481481481481e-2_f64 * t5610 + t8396 * t548 + 0.74498e-1_f64 * t4347 * t8289 - 0.23214722222222222222e-2_f64 * t8173 + 0.15476481481481481481e-2_f64 * t8178 - 0.23214722222222222222e-2_f64 * t8182;
    t8431
}
