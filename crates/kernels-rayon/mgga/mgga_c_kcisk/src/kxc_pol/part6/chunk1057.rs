//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1057/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1057(t31295: f64, t487: f64, t1487: f64, t30205: f64, t382: f64, t486: f64, t2275: f64, t27204: f64, t1471: f64, t2059: f64, t27331: f64, t30153: f64, t4272: f64) -> (f64, f64, f64, f64, f64) {
    let t31296 = t487 * t31295;
    let t31297 = t1487 * t31296;
    let t31299 = t382 * t30205;
    let t31300 = t487 * t31299;
    let t31301 = t486 * t31300;
    let t31303 = t27204 * t2275;
    let t31324 = t1471 * t27331 * t2059;
    let t31328 = t1471 * t4272 * t30153;
    (t31297, t31301, t31303, t31324, t31328)
}
