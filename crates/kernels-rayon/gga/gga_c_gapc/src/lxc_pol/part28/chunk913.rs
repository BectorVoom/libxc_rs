//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 913/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk913(t10360: f64, t284: f64, t10142: f64, t876: f64, t2902: f64, t932: f64, t1055: f64, t787: f64, t10326: f64, t10330: f64, t10333: f64, t10337: f64, t10341: f64, t10344: f64, t10351: f64, t10358: f64) -> (f64, f64, f64, f64, f64) {
    let t10361 = t284 * t10360;
    let t10363 = t10142 * t876;
    let t10364 = t284 * t10363;
    let t10366 = t2902 * t932;
    let t10367 = t1055 * t787;
    let t10368 = t10366 * t10367;
    let t10370 = 0.11742981196020707897e-4_f64 * t10326 + 0.342503618217270647e-5_f64 * t10330 - 0.11742981196020707897e-4_f64 * t10333 + 0.39896999657995323756e-6_f64 * t10337 + 0.82073827867876094584e-5_f64 * t10341 - 0.23938199794797194254e-5_f64 * t10344 - 0.39896999657995323756e-6_f64 * t10351 - 0.39896999657995323756e-6_f64 * t10358 - 0.28183154870449698953e-3_f64 * t10361 - 0.56366309740899397906e-3_f64 * t10364 + 0.27357942622625364862e-5_f64 * t10368;
    (t10361, t10364, t10366, t10368, t10370)
}
