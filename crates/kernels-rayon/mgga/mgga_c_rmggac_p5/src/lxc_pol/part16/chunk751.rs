//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 751/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk751(t1323: f64, t35206: f64, t7761: f64, t7556: f64, t934: f64, t270: f64, t356: f64, t290: f64, t31: f64, t2019: f64, t640: f64, t7764: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35207 = t1323 * t35206;
    let t35208 = t35207 * t7761;
    let t35210 = t934 * t7556;
    let t35214 = t356 * t270;
    let t35215 = t290 * t35214;
    let t35219 = t356 * t31;
    let t35220 = t290 * t35219;
    let t35226 = t2019 * t7764 * t640 * t35214;
    (t35207, t35208, t35210, t35214, t35215, t35219, t35220, t35226)
}
