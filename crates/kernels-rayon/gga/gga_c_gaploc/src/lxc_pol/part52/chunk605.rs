//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 605/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk605(t1628: f64, t3581: f64, t11471: f64, t11476: f64, t11482: f64, t11485: f64, t11490: f64, t11493: f64, t1532: f64, t1562: f64, t1580: f64, t1599: f64, t1641: f64, t193: f64, t3570: f64, t3573: f64, t3577: f64, t3582: f64, t3586: f64, t4950: f64, t557: f64, t574: f64, t597: f64) -> f64 {
    let t11496 = t1628 * t3581;
    let t11499 = 0.11502877786176224903e2_f64 * t1580 * t3582 - 0.23005755572352449806e1_f64 * t1641 * t3586 - 0.23005755572352449806e1_f64 * t574 * t11471 - 0.35750489951850426669e0_f64 * t1599 * t3573 - 0.35750489951850426669e0_f64 * t557 * t11476 - 0.46011511144704899612e1_f64 * t1641 * t3577 + 0.35750489951850426669e0_f64 * t11482 * t193 - 0.10725146985555128001e1_f64 * t11485 * t1532 + 0.71500979903700853338e0_f64 * t4950 * t3570 + 0.30674340763136599741e1_f64 * t597 * t11490 - 0.92023022289409799224e1_f64 * t1562 * t11493 + 0.15337170381568299871e2_f64 * t597 * t11496;
    t11499
}
