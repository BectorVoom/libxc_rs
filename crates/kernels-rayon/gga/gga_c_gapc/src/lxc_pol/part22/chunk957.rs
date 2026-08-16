//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 957/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk957(t10371: f64, t10374: f64, t10376: f64, t10379: f64, t10383: f64, t10386: f64, t10390: f64, t10393: f64, t10396: f64, t10399: f64, t10403: f64, t11060: f64, t11072: f64, t11085: f64, t11097: f64, t11111: f64, t11123: f64, t11136: f64) -> f64 {
    let t11148 = -0.7113065081882594864e-4_f64 * t10371 + 0.82073827867876094584e-5_f64 * t10374 + 0.18788769913633132635e-2_f64 * t10376 + 0.8768092626362128563e-4_f64 * t10379 - 0.11273261948179879581e-2_f64 * t10383 + 0.18788769913633132635e-2_f64 * t10386 - 0.26597999771996882504e-6_f64 * t10390 - 0.82073827867876094584e-5_f64 * t10393 - 0.16414765573575218917e-4_f64 * t10396 - 0.82073827867876094584e-5_f64 * t10399 - 0.23485962392041415794e-4_f64 * t10403;
    let t11151 = t11060 + t11072 + t11085 + t11097 + t11111 + t11123 + t11136 + t11148;
    t11151
}
