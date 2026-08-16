//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta250 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1128;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1129;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1130;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1131;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta250(t1891: f64, t6597: f64, t133: f64, t119: f64, t212: f64, t1895: f64, t213: f64, t225: f64, t1892: f64, t815: f64, t829: f64, t1898: f64, t808: f64, t249: f64, t59: f64, t814: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6598, t6600, t6601, t6603, t6604) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1128(t1891, t6597, t133, t119, t212, t1895, t213, t225);
        let t6605 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1129(t1892, t6604);
        let (t6606, t6607, t6609, t6610, t6612) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1130(t815, t829, t6605, t1898, t808, t249, t59, t814);
        let t6613 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1131(t240, t6612);
    (t6598, t6600, t6601, t6603, t6604, t6605, t6606, t6607, t6609, t6610, t6612, t6613)
}
