//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1029/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1029(t14078: f64, t14081: f64, t14085: f64, t10243: f64, t1240: f64, t13448: f64, t14062: f64, t14065: f64, t14070: f64, t14075: f64, t15168: f64, t15611: f64) -> f64 {
    let t15632 = 0.23214722222222222222e-2_f64 * t14078;
    let t15638 = 0.30952962962962962962e-2_f64 * t14081;
    let t15639 = 0.15476481481481481481e-2_f64 * t14085;
    let t15640 = -0.92858888888888888886e-2_f64 * t13448 + 0.17411041666666666666e-2_f64 * t14062 - 0.38691203703703703703e-3_f64 * t14065 - 0.51588271604938271604e-3_f64 * t14070 + 0.46429444444444444443e-2_f64 * t14075 - t15632 - 0.13345e0_f64 * t1240 * t15611 + 0.66725e-1_f64 * t1240 * t15168 - 0.41270617283950617284e-2_f64 * t10243 - t15638 - t15639;
    t15640
}
