//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta65 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk412;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk413;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta65<F: Float>(t1015: F, t1469: F, t1012: F, t1647: F, t225: F, t366: F, t1651: F, t373: F, t372: F, t371: F, t1598: F, t1612: F, t1638: F, t1640: F, t1644: F, t1045: F, t1042: F, t1066: F, t1592: F, t247: F, t1009: F, t1011: F, t1025: F, t1041: F, t1060: F, t1063: F, t375: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1655, t1656, t1659, t1660, t1663, t1665, t1668) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk412::<F>(t1015, t1469, t1012, t1647, t225, t366, t1651, t373, t372, t371, t1598, t1612, t1638, t1640, t1644);
        let (t1670, t1671, t1675, t1678) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk413::<F>(t1668, t373, t1045, t1042, t1066, t1592, t247, t1009, t1011, t1025, t1041, t1060, t1063, t1656, t1660, t1665, t375);
    (t1655, t1659, t1660, t1663, t1665, t1668, t1670, t1671, t1675, t1678)
}
