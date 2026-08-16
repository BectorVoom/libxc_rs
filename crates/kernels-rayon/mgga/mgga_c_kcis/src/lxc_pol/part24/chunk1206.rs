//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1206/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1206(t26972: f64, t8083: f64, t96005: f64, t96018: f64, t26966: f64, t28214: f64, t28219: f64, t7784: f64, t7772: f64, t97024: f64, t96940: f64, t1250: f64, t15198: f64, t251: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t97166 = t8083 * t26972;
    let t97173 = 0.15476481481481481481e-2_f64 * t96005;
    let t97193 = 0.23214722222222222222e-2_f64 * t96018;
    let t97212 = t26966 * t28214;
    let t97248 = 0.23168402777777777778e-3_f64 * t28219 * t7784;
    let t97250 = 0.30918233506944444444e-4_f64 * t7772 * t97024;
    let t97265 = 0.30918233506944444444e-4_f64 * t7772 * t96940;
    let t97267 = t15198 * t251 * t1250;
    (t97166, t97173, t97193, t97212, t97248, t97250, t97265, t97267)
}
