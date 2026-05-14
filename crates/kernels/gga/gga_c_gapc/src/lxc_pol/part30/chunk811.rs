//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 811/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk811<F: Float>(t10326: F, t10330: F, t10333: F, t10337: F, t10341: F, t10344: F, t10351: F, t10358: F, t10361: F, t10364: F, t10368: F, t10371: F, t10374: F, t10376: F, t10379: F, t10383: F, t10386: F, t10390: F, t10393: F, t10396: F, t10399: F, t10403: F) -> (F, F) {
    let t11136 = 0.23485962392041415794e-4 * t10326 + 0.685007236434541294e-5 * t10330 - 0.23485962392041415794e-4 * t10333 + 0.79793999315990647512e-6 * t10337 + 0.16414765573575218917e-4 * t10341 - 0.47876399589594388508e-5 * t10344 - 0.79793999315990647512e-6 * t10351 - 0.79793999315990647512e-6 * t10358 - 0.56366309740899397906e-3 * t10361 - 0.11273261948179879581e-2 * t10364 + 0.54715885245250729722e-5 * t10368;
    let t11148 = -0.7113065081882594864e-4 * t10371 + 0.82073827867876094584e-5 * t10374 + 0.18788769913633132635e-2 * t10376 + 0.8768092626362128563e-4 * t10379 - 0.11273261948179879581e-2 * t10383 + 0.18788769913633132635e-2 * t10386 - 0.26597999771996882504e-6 * t10390 - 0.82073827867876094584e-5 * t10393 - 0.16414765573575218917e-4 * t10396 - 0.82073827867876094584e-5 * t10399 - 0.23485962392041415794e-4 * t10403;
    (t11136, t11148)
}
