//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 349/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk349<F: Float>(t1539: F, t1629: F, t1160: F, t1533: F, t1251: F, t525: F, t1411: F, t456: F, t407: F, t310: F, t553: F, t159: F, t545: F, t322: F, t381: F, t394: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1630 = t1629 * t1539;
    let t1631 = t1160 * t1630;
    let t1633 = t1629 * t1533;
    let t1636 = t1251 * t525;
    let t1639 = t456 * t1411;
    let t1642 = t1629 * t407;
    let t1645 = t310 * t553;
    let t1647 = t159 * t545;
    let t1648 = t1647 * t322;
    let t1649 = t381 * t1648;
    let t1651 = t394 * t545;
    (t1630, t1631, t1633, t1636, t1639, t1642, t1645, t1647, t1648, t1649, t1651)
}
