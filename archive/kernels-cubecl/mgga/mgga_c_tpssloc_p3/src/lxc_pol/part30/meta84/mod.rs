//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta84 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk545;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk546;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk547;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk548;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta84<F: Float>(t1129: F, t1148: F, t1659: F, t1673: F, t1675: F, t1683: F, t1688: F, t1695: F, t300: F, t436: F, t1147: F, t1156: F, t1694: F, t1164: F, t1420: F, t338: F, t1178: F, t1409: F, t1177: F, t1111: F, t1668: F, t457: F, t460: F, t974: F, t1173: F, t1174: F, t463: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1699, t1701, t1703) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk545::<F>(t1129, t1148, t1659, t1673, t1675, t1683, t1688, t1695, t300, t436, t1147, t1156, t1694);
        let (t1705, t1706) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk546::<F>(t1164, t1703, t1420, t338);
        let (t1709, t1710, t1714) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk547::<F>(t1178, t1409, t1177, t1111, t1668);
        let (t1716, t1717, t1720) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk548::<F>(t1714, t457, t460, t974, t1173, t1174, t1706, t1710, t463);
    (t1699, t1701, t1703, t1705, t1706, t1709, t1710, t1714, t1716, t1717, t1720)
}
