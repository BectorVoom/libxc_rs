//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 399/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk399(t1532: f64, t1562: f64, t1572: f64, t193: f64, t3194: f64, t3375: f64, t3382: f64, t3397: f64, t3408: f64, t3557: f64, t3561: f64, t3566: f64, t3570: f64, t3573: f64, t3577: f64, t3582: f64, t3586: f64, t3592: f64, t3596: f64, t557: f64, t574: f64, t597: f64) -> f64 {
    let t3599 = 0.35750489951850426669e0_f64 * t3557 * t193 + 0.35750489951850426669e0_f64 * t3561 * t193 + 0.59584149919750711116e-1_f64 * t3375 - 0.10725146985555128001e1_f64 * t3566 * t1532 - 0.59584149919750711116e-1_f64 * t3382 + 0.71500979903700853338e0_f64 * t1572 * t3570 - 0.35750489951850426669e0_f64 * t557 * t3573 - 0.46011511144704899612e1_f64 * t574 * t3577 - 0.76685851907841499353e0_f64 * t3397 + 0.11502877786176224903e2_f64 * t597 * t3582 - 0.23005755572352449806e1_f64 * t574 * t3586 - 0.31952438294933958063e-1_f64 * t3194 + 0.76685851907841499353e0_f64 * t3408 - 0.69017266717057349418e1_f64 * t1562 * t3592 + 0.23005755572352449806e1_f64 * t597 * t3596;
    t3599
}
