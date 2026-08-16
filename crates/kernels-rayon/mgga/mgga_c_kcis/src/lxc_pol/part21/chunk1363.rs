//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1363/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1363(t11061: f64, t7788: f64, t8090: f64, t26972: f64, t8083: f64, t2197: f64, t26960: f64, t27083: f64, t5345: f64, t7772: f64, t92941: f64, t92943: f64, t92946: f64, t92951: f64, t92955: f64, t92958: f64, t96968: f64, t97039: f64) -> f64 {
    let t97153 = t7788 * t11061 * t8090;
    let t97166 = t8083 * t26972;
    let t97170 = 0.25742669753086419753e-4_f64 * t97153 - 0.2782641015625e-3_f64 * t7772 * t97039 + 0.20594135802469135802e-3_f64 * t92941 - 0.15476481481481481481e-2_f64 * t92943 + 0.11584201388888888889e-3_f64 * t92946 - 0.82448622685185185185e-4_f64 * t92951 + 0.23168402777777777778e-3_f64 * t92955 + 0.23168402777777777778e-3_f64 * t92958 - 0.33980324074074074074e-2_f64 * t5345 * t27083 * t2197 + 0.7722800925925925926e-4_f64 * t97166 - 0.23168402777777777778e-3_f64 * t26960 * t96968;
    t97170
}
