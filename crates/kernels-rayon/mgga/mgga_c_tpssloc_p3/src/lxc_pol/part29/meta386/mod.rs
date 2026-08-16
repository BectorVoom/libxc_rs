//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta386 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1573;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1574;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1575;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1576;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1577;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1578;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta386(t1088: f64, t14706: f64, t123: f64, t4778: f64, t699: f64, t1113: f64, t136: f64, t4725: f64, t690: f64, t4730: f64, t14704: f64, t11147: f64, t1409: f64, t2244: f64, t11145: f64, t11153: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t14708 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1573(t1088, t14706, t123);
        let (t14710, t14711, t14713, t14720) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1574(t4778, t699, t1113, t14706, t136, t4725, t690);
        let (t14721, t14722) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1575(t14720, t4730, t690);
        let (t14723, t14724, t14726) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1576(t14722, t14704, t11147, t1409, t2244);
        let t14728 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1577(t11145, t14726, t123);
        let t14731 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1578(t11153, t1409, t2244);
    (t14708, t14710, t14711, t14713, t14720, t14721, t14722, t14723, t14724, t14726, t14728, t14731)
}
