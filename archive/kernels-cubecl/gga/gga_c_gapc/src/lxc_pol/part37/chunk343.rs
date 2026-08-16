//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 343/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk343<F: Float>(t6: F, t644: F, t101: F, t517: F, t423: F, t462: F, t472: F, t513: F, t1468: F, t465: F, t1427: F, t433: F) -> (F, F, F, F, F, F, F) {
    let t1482 = t6 * t644;
    let t1483 = t1482 * t101;
    let t1484 = t1483 * t517;
    let t1487 = t462 * t423;
    let t1488 = t1487 * t472;
    let t1491 = t513 * t423;
    let t1492 = t1491 * t517;
    let t1495 = t1468 * t465;
    let t1498 = t1427 * t433;
    (t1482, t1484, t1487, t1488, t1492, t1495, t1498)
}
