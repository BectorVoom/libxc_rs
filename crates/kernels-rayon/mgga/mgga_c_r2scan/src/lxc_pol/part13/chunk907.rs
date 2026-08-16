//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 907/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk907(t1216: f64, t313: f64, t6678: f64, t806: f64, t810: f64, t8316: f64, t8323: f64, t8326: f64, t8329: f64, t8337: f64, t8344: f64, t8347: f64, t8350: f64, t8377: f64, t8385: f64) -> f64 {
    let t8395 = 3.0_f64 / 10.0_f64 * t313 * (-10.0_f64 / 27.0_f64 * t8316 + 20.0_f64 / 9.0_f64 * t8377 * t1216 * t806 + 10.0_f64 / 9.0_f64 * t8323 + 5.0_f64 / 3.0_f64 * t8326 - 5.0_f64 * t8329 - 10.0_f64 / 27.0_f64 * t8337 - 20.0_f64 / 9.0_f64 * t8385 * t1216 * t810 + 10.0_f64 / 9.0_f64 * t8344 - 5.0_f64 / 3.0_f64 * t8347 + 5.0_f64 * t8350) - t6678;
    t8395
}
