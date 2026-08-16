//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta247 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1039;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1040;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1041;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1042;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1043;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta247(t815: f64, t829: f64, t6605: f64, t1898: f64, t808: f64, t249: f64, t59: f64, t814: f64, t240: f64, t812: f64, t831: f64, t1899: f64, t838: f64, t234: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6606, t6607, t6609, t6610, t6612) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1039(t815, t829, t6605, t1898, t808, t249, t59, t814);
        let t6613 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1040(t240, t6612);
        let t6614 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1041(t6613, t812);
        let (t6615, t6617, t6619, t6620) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1042(t6614, t831, t1899, t838, t234, t59, t240);
        let t6621 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1043(t6620, t812);
    (t6606, t6607, t6609, t6610, t6612, t6613, t6614, t6615, t6617, t6619, t6620, t6621)
}
