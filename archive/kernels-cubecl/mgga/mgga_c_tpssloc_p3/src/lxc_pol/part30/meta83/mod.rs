//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta83 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk540;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk541;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk542;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk543;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk544;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta83<F: Float>(t1100: F, t1661: F, t1107: F, t1113: F, t1653: F, t136: F, t1105: F, t1112: F, t1655: F, t1118: F, t1099: F, t1122: F, t1131: F, t1134: F, t1137: F, t1141: F, t449: F, t1150: F, t1153: F, t1156: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1662, t1665, t1667, t1668, t1670, t1671) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk540::<F>(t1100, t1661, t1107, t1113, t1653, t136, t1105, t1112, t1655, t1118);
        let (t1673, t1675, t1682, t1683) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk541::<F>(t1099, t1671, t1122, t1655, t1131, t1134, t1662, t1665, t1668, t1137);
        let t1687 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk542::<F>(t1141, t1655);
        let (t1688, t1694) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk543::<F>(t1687, t449, t1150, t1153, t1655, t1662, t1665, t1668);
        let t1695 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk544::<F>(t1156, t1694);
    (t1667, t1668, t1670, t1671, t1673, t1675, t1682, t1683, t1687, t1688, t1694, t1695)
}
