//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 359/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk359<F: Float>(t1485: F, t49: F, t55: F, t63: F, t1479: F, t1482: F, t482: F, t1478: F, t50: F, t65: F, t1480: F, t1483: F) -> (F, F, F, F, F, F) {
    let t1486 = t49 * t1485;
    let t1488 = 1.0/f64::sqrt(t55);
    let t1489 = t1488 * t63;
    let t1490 = t1489 * t1479;
    let t1492 = t482 * t1482;
    let t1495 = t65 * t50 * t1478;
    let t1497 = -0.57538888888888888889e0 * t1480 + 0.11507777777777777778e1 * t1483 + 0.40256666666666666667e0 * t1486 + 0.366775e-1 * t1490 + 0.73355e-1 * t1492 + 0.137975e0 * t1495;
    (t1486, t1489, t1490, t1492, t1495, t1497)
}
