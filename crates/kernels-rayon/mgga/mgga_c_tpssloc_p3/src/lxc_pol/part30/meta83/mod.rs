//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta83 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk540;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk541;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk542;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk543;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk544;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta83(t1100: f64, t1661: f64, t1107: f64, t1113: f64, t1653: f64, t136: f64, t1105: f64, t1112: f64, t1655: f64, t1118: f64, t1099: f64, t1122: f64, t1131: f64, t1134: f64, t1137: f64, t1141: f64, t449: f64, t1150: f64, t1153: f64, t1156: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1662, t1665, t1667, t1668, t1670, t1671) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk540(t1100, t1661, t1107, t1113, t1653, t136, t1105, t1112, t1655, t1118);
        let (t1673, t1675, t1682, t1683) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk541(t1099, t1671, t1122, t1655, t1131, t1134, t1662, t1665, t1668, t1137);
        let t1687 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk542(t1141, t1655);
        let (t1688, t1694) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk543(t1687, t449, t1150, t1153, t1655, t1662, t1665, t1668);
        let t1695 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk544(t1156, t1694);
    (t1667, t1668, t1670, t1671, t1673, t1675, t1682, t1683, t1687, t1688, t1694, t1695)
}
