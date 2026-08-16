//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1349/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1349(t27042: f64, t28093: f64, t46978: f64, t7788: f64, t8094: f64, t95826: f64, t1259: f64, t417: f64, t15610: f64, t26997: f64, t26954: f64, t28189: f64) -> (f64, f64, f64, f64, f64) {
    let t96899 = t27042 * t28093;
    let t96902 = t7788 * t46978 * t8094;
    let t96904 = 0.15476481481481481481e-2_f64 * t95826;
    let t96908 = t417 * t1259;
    let t96910 = t96908 * t15610 * t26997;
    let t96917 = t28189 * t26954;
    (t96899, t96902, t96904, t96910, t96917)
}
