//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1021/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1021(t40771: f64, t9147: f64, t10066: f64, t34764: f64, t2298: f64, t26370: f64, t17859: f64, t9051: f64, t9055: f64, t9096: f64, t9138: f64, t2310: f64, t38472: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47333 = t40771 * t9147;
    let t47335 = t34764 * t10066;
    let t47340 = t26370 * t2298;
    let t47345 = t17859 * t9051;
    let t47347 = t17859 * t9055;
    let t47349 = t17859 * t9096;
    let t47351 = t17859 * t9138;
    let t47353 = t38472 * t2310;
    (t47333, t47335, t47340, t47345, t47347, t47349, t47351, t47353)
}
