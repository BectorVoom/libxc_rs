//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1200/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1200(t95587: f64, t1250: f64, t251: f64, t47652: f64, t2888: f64, t7773: f64, t46978: f64, t8086: f64, t7772: f64, t15553: f64, t28145: f64, t7788: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96787 = 0.15476481481481481481e-2_f64 * t95587;
    let t96790 = t47652 * t251 * t1250;
    let t96793 = t2888 * t7773;
    let t96812 = t46978 * t8086;
    let t96813 = t7772 * t96812;
    let t96836 = t7788 * t15553 * t28145;
    (t96787, t96790, t96793, t96812, t96813, t96836)
}
