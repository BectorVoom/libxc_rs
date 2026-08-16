//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1383/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1383(t191: f64, t4529: f64, t34378: f64, t34506: f64, t10517: f64, t7014: f64, t10615: f64, t31167: f64, t6703: f64, t8248: f64, t10578: f64, t10590: f64, t1508: f64, t1599: f64, t193: f64, t30712: f64, t30735: f64, t31623: f64, t3371: f64, t3387: f64, t3402: f64, t34498: f64, t34500: f64, t34503: f64, t4598: f64, t4631: f64, t524: f64, t568: f64, t569: f64, t574: f64) -> f64 {
    let t34507 = t191 * t4529;
    let t34510 = 0.85801175884441024004e1_f64 * t34506 * t34507 * t34378;
    let t34512 = 0.87421871174939309262e2_f64 * t7014 * t10517;
    let t34530 = t10615 * t31167;
    let t34531 = 0.44688112439813033337e-1_f64 * t34530;
    let t34533 = 0.2780593662921699852e0_f64 * t8248 * t6703;
    let t34534 = -t34498 - t34500 - t34503 + t34510 + t34512 - t30712 - 0.23005755572352449806e1_f64 * t574 * t568 * t569 * t31623 + 0.35750489951850426669e0_f64 * t1508 * t3371 * t193 + 0.71500979903700853338e0_f64 * t524 * t10590 * t193 - 0.35750489951850426669e0_f64 * t4631 * t3387 - 0.71500979903700853338e0_f64 * t1599 * t10578 - 0.1022478025437886658e1_f64 * t574 * t4598 * t3402 - t34531 - t30735 - t34533;
    t34534
}
