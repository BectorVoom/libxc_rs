//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 920/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk920(t12282: f64, t1336: f64, t3777: f64, t3789: f64, t12248: f64, t236: f64, t3798: f64, t12189: f64, t1329: f64, t1333: f64, t3862: f64, t10022: f64, t248: f64, t557: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12283 = t1336 * t12282;
    let t12286 = t3777 * t3789;
    let t12289 = t12248 * t236;
    let t12300 = t3777 * t3798;
    let t12308 = t12189 * t1329;
    let t12325 = t1333 * t3862;
    let t12328 = t10022 * t557 * t248;
    (t12283, t12286, t12289, t12300, t12308, t12325, t12328)
}
