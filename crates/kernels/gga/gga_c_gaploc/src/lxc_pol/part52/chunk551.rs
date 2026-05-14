//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 551/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk551<F: Float>(t11218: F, t189: F, t188: F, t3565: F, t524: F, t1628: F, t3595: F, t3591: F, t3581: F, t11471: F, t11476: F, t1532: F, t1562: F, t1580: F, t1599: F, t1641: F, t193: F, t3570: F, t3573: F, t3577: F, t3582: F, t3586: F, t4950: F, t557: F, t574: F, t597: F) -> (F, F, F) {
    let t11481 = t189 * t11218;
    let t11482 = t188 * t11481;
    let t11485 = t524 * t3565;
    let t11490 = t1628 * t3595;
    let t11493 = t1628 * t3591;
    let t11496 = t1628 * t3581;
    let t11499 = 0.11502877786176224903e2 * t1580 * t3582 - 0.23005755572352449806e1 * t1641 * t3586 - 0.23005755572352449806e1 * t574 * t11471 - 0.35750489951850426669e0 * t1599 * t3573 - 0.35750489951850426669e0 * t557 * t11476 - 0.46011511144704899612e1 * t1641 * t3577 + 0.35750489951850426669e0 * t11482 * t193 - 0.10725146985555128001e1 * t11485 * t1532 + 0.71500979903700853338e0 * t4950 * t3570 + 0.30674340763136599741e1 * t597 * t11490 - 0.92023022289409799224e1 * t1562 * t11493 + 0.15337170381568299871e2 * t597 * t11496;
    (t11481, t11485, t11499)
}
