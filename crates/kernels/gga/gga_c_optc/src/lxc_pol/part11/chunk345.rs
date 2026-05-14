//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 345/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk345<F: Float>(t1506: F, t429: F, t438: F, t914: F, t146: F, t1497: F, t455: F, t1502: F, t449: F, t894: F, t1514: F, t464: F, t155: F, t1150: F, t1159: F, t1162: F, t1170: F, t1177: F, t1179: F, t1520: F, t451: F, t459: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1527 = t429 * t1506;
    let t1528 = t1527 * t438;
    let t1529 = t914 * t1528;
    let t1533 = t146 * t455 * t1497;
    let t1536 = t914 * t1502;
    let t1539 = t449 * t1506;
    let t1540 = t1539 * t438;
    let t1541 = t894 * t1540;
    let t1544 = t464 * t1514;
    let t1545 = t155 * t1544;
    let t1550 = 0.11360101276506094136e1 * t1150 * t1529 - 0.23181763972770020946e0 * t1533 * t459 + t1159 + 0.28977204965962526182e-1 * t1162 * t1536 + 0.5848048239485271795e1 * t1170 * t1541 - 0.4030456356669135783e-1 * t1545 * t451 + t1177 + 0.50380704458364197288e-2 * t1179 * t1520;
    (t1528, t1529, t1533, t1536, t1540, t1541, t1544, t1545, t1550)
}
