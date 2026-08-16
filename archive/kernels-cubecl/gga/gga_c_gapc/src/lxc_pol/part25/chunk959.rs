//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 959/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk959<F: Float>(t10371: F, t10374: F, t10376: F, t10379: F, t10383: F, t10386: F, t10390: F, t10393: F, t10396: F, t10399: F, t10403: F, t11060: F, t11072: F, t11085: F, t11097: F, t11111: F, t11123: F, t11136: F) -> F {
    let t11148 = -F::cast_from(0.7113065081882594864e-4_f64) * t10371 + F::cast_from(0.82073827867876094584e-5_f64) * t10374 + F::cast_from(0.18788769913633132635e-2_f64) * t10376 + F::cast_from(0.8768092626362128563e-4_f64) * t10379 - F::cast_from(0.11273261948179879581e-2_f64) * t10383 + F::cast_from(0.18788769913633132635e-2_f64) * t10386 - F::cast_from(0.26597999771996882504e-6_f64) * t10390 - F::cast_from(0.82073827867876094584e-5_f64) * t10393 - F::cast_from(0.16414765573575218917e-4_f64) * t10396 - F::cast_from(0.82073827867876094584e-5_f64) * t10399 - F::cast_from(0.23485962392041415794e-4_f64) * t10403;
    let t11151 = t11060 + t11072 + t11085 + t11097 + t11111 + t11123 + t11136 + t11148;
    t11151
}
