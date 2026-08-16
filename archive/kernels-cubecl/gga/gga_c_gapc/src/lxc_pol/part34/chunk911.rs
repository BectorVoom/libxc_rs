//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 911/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk911<F: Float>(t10360: F, t284: F, t10142: F, t876: F, t2902: F, t932: F, t1055: F, t787: F, t10326: F, t10330: F, t10333: F, t10337: F, t10341: F, t10344: F, t10351: F, t10358: F) -> (F, F, F, F, F) {
    let t10361 = t284 * t10360;
    let t10363 = t10142 * t876;
    let t10364 = t284 * t10363;
    let t10366 = t2902 * t932;
    let t10367 = t1055 * t787;
    let t10368 = t10366 * t10367;
    let t10370 = F::cast_from(0.11742981196020707897e-4_f64) * t10326 + F::cast_from(0.342503618217270647e-5_f64) * t10330 - F::cast_from(0.11742981196020707897e-4_f64) * t10333 + F::cast_from(0.39896999657995323756e-6_f64) * t10337 + F::cast_from(0.82073827867876094584e-5_f64) * t10341 - F::cast_from(0.23938199794797194254e-5_f64) * t10344 - F::cast_from(0.39896999657995323756e-6_f64) * t10351 - F::cast_from(0.39896999657995323756e-6_f64) * t10358 - F::cast_from(0.28183154870449698953e-3_f64) * t10361 - F::cast_from(0.56366309740899397906e-3_f64) * t10364 + F::cast_from(0.27357942622625364862e-5_f64) * t10368;
    (t10361, t10364, t10366, t10368, t10370)
}
