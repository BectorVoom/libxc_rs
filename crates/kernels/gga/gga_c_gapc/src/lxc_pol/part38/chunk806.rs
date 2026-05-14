//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 806/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk806<F: Float>(t10363: F, t284: F, t2902: F, t932: F, t1055: F, t787: F, t10326: F, t10330: F, t10333: F, t10337: F, t10341: F, t10344: F, t10351: F, t10358: F, t10361: F, t10102: F, t1058: F) -> (F, F, F) {
    let t10364 = t284 * t10363;
    let t10366 = t2902 * t932;
    let t10367 = t1055 * t787;
    let t10368 = t10366 * t10367;
    let t10370 = 0.11742981196020707897e-4 * t10326 + 0.342503618217270647e-5 * t10330 - 0.11742981196020707897e-4 * t10333 + 0.39896999657995323756e-6 * t10337 + 0.82073827867876094584e-5 * t10341 - 0.23938199794797194254e-5 * t10344 - 0.39896999657995323756e-6 * t10351 - 0.39896999657995323756e-6 * t10358 - 0.28183154870449698953e-3 * t10361 - 0.56366309740899397906e-3 * t10364 + 0.27357942622625364862e-5 * t10368;
    let t10371 = t10102 * t1058;
    (t10366, t10370, t10371)
}
