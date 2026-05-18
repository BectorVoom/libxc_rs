//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 436/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk436<F: Float>(t1734: F, t568: F, t179: F, t1501: F, t1510: F, t1513: F, t1520: F, t1555: F, t1557: F, t1559: F, t1627: F, t1630: F, t1632: F, t1663: F) -> (F, F) {
    let t1735 = t1734 * t568;
    let t1736 = t179 * t1735;
    let t1739 = t1627 - t1501 - t1510 - t1513 + t1663 + t1630 - t1632 - t1555 + t1557 + t1559 - t1520;
    (t1736, t1739)
}
