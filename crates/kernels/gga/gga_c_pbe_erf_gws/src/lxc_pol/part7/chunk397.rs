//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 397/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk397<F: Float>(t1663: F, t197: F, t1403: F, t1661: F, t587: F, t708: F, t723: F, t1615: F, t1619: F, t1626: F, t1629: F, t1633: F, t1637: F, t1647: F, t1650: F, t1654: F, t1658: F) -> (F, F, F, F, F) {
    let t1664 = t197 * t1663;
    let t1665 = t1664 * t1403;
    let t1666 = t1661 * t1665;
    let t1668 = 4.0 / 27.0 * t587 * t1666;
    let t1669 = t708 * t723;
    let t1671 = -4.0 / 45.0 * t1615 + t1619 - t1626 + t1629 + t1633 + t1637 + t1647 + t1650 + t1654 + t1658 + t1668 + 4.0 / 9.0 * t1669;
    (t1664, t1665, t1666, t1668, t1671)
}
