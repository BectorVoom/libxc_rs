//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta85 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk549;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk550;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk551;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk552;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk553;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta85(t1720: f64, t491: f64, t1196: f64, t1409: f64, t974: f64, t225: f64, t68: f64, t484: f64, t1659: f64, t1673: f64, t1699: f64, t1701: f64, t1705: f64, t475: f64, t1214: f64, t248: f64, t46: f64, t480: f64, t47: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1721, t1725, t1726, t1729) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk549(t1720, t491, t1196, t1409, t974, t225);
        let (t1730, t1731, t1734) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk550(t1729, t68, t484, t1659, t1673, t1699, t1701, t1705);
        let t1735 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk551(t1734, t475);
        let t1737 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk552(t1214, t1735, t248);
        let t1742 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk553(t46, t480, t47);
    (t1721, t1725, t1726, t1729, t1730, t1731, t1734, t1735, t1737, t1742)
}
