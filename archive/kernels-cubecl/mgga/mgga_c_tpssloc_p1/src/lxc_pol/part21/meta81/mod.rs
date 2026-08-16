//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta81 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk582;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk583;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk584;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk585;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk586;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk587;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk588;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk589;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta81<F: Float>(t1099: F, t1671: F, t1122: F, t1655: F, t1131: F, t1134: F, t1662: F, t1665: F, t1668: F, t1137: F, t1141: F, t449: F, t1150: F, t1153: F, t1156: F, t1129: F, t1148: F, t1659: F, t300: F, t436: F, t1147: F, t1164: F, t1420: F, t338: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1673, t1675) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk582::<F>(t1099, t1671, t1122, t1655);
        let t1682 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk583::<F>(t1131, t1134, t1655, t1662, t1665, t1668);
        let t1683 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk584::<F>(t1137, t1682);
        let t1687 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk585::<F>(t1141, t1655);
        let (t1688, t1694) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk586::<F>(t1687, t449, t1150, t1153, t1655, t1662, t1665, t1668);
        let t1695 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk587::<F>(t1156, t1694);
        let (t1699, t1701, t1703) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk588::<F>(t1129, t1148, t1659, t1673, t1675, t1683, t1688, t1695, t300, t436, t1147, t1156, t1694);
        let (t1705, t1706) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk589::<F>(t1164, t1703, t1420, t338);
    (t1673, t1675, t1682, t1683, t1687, t1694, t1695, t1699, t1701, t1703, t1705, t1706)
}
