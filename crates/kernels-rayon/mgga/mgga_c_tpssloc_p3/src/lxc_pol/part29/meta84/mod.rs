//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta84 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk546;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk547;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk548;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk549;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk550;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk551;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta84(t1178: f64, t1409: f64, t1177: f64, t1111: f64, t1668: f64, t457: f64, t460: f64, t974: f64, t1173: f64, t1174: f64, t1706: f64, t463: f64, t491: f64, t1196: f64, t225: f64, t68: f64, t484: f64, t1659: f64, t1673: f64, t1699: f64, t1701: f64, t1705: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1709, t1710, t1714) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk546(t1178, t1409, t1177, t1111, t1668);
        let t1715 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk547(t1714, t457);
        let t1716 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk548(t1715, t460);
        let t1720 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk549(t1716, t974, t1173, t1174, t1706, t1710, t463);
        let (t1721, t1725, t1726, t1729) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk550(t1720, t491, t1196, t1409, t974, t225);
        let (t1730, t1731, t1734) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk551(t1729, t68, t484, t1659, t1673, t1699, t1701, t1705);
    (t1709, t1714, t1715, t1716, t1720, t1721, t1725, t1726, t1729, t1730, t1731, t1734)
}
