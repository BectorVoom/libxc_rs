//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1332/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1332<F: Float>(t2893: F, t4576: F, t1849: F, t502: F, t1572: F, t31247: F, t513: F, t1129: F, t11552: F, t11520: F, t11342: F, t9507: F, t11536: F, t11544: F, t26552: F, t26564: F, t26579: F, t31225: F, t31229: F, t9632: F, t9642: F, t9667: F, t9765: F, t9769: F, t9773: F) -> (F, F) {
    let t31304 = t4576 * t2893;
    let t31309 = t502 * t1849;
    let t31310 = t31309 * t1572;
    let t31311 = t31247 * t513;
    let t31317 = t11552 * t1129;
    let t31322 = t11520 * t1129;
    let t31330 = t11342 * t9507;
    let t31337 = 504.0 * t9773 * t31304 + 24.0 * t9765 * t31304 + 10000.0 / 81.0 * t31310 * t31311 - 360.0 * t9769 * t11544 * t1129 + 504.0 * t9773 * t31317 + 24.0 * t9765 * t31317 - 96.0 * t26552 * t31322 - 1440.0 * t26579 * t11536 * t1129 - 4032.0 * t26564 * t31322 + 1408.0 / 81.0 * t9642 * t31330 - 6400.0 / 81.0 * t9632 * t31229 - 1408.0 / 243.0 * t9667 * t31225;
    (t31330, t31337)
}
