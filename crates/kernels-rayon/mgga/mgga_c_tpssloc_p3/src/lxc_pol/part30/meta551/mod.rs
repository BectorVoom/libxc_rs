//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1906;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1907;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1908;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta551(t22761: f64, t6390: f64, t2002: f64, t6378: f64, t559: f64, t6422: f64, t6945: f64, t6427: f64, t6952: f64, t6431: f64, t1831: f64, t26257: f64, t1799: f64, t1824: f64, t550: f64, t1339: f64, t22827: f64, t22833: f64, t6396: f64, t22820: f64, t22826: f64, t22859: f64, t22864: f64, t22868: f64, t26272: f64, t26295: f64, t28083: f64, t539: f64, t2015: f64, t6460: f64, t3887: f64, t1842: f64, t26337: f64, t22635: f64, t22633: f64, t1825: f64, t26421: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28085, t28088, t28089, t28091, t28093, t28095, t28097) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1906(t22761, t6390, t2002, t6378, t559, t6422, t6945, t6427, t6952, t6431, t1831, t26257);
        let (t28100, t28101, t28106) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1907(t1799, t1824, t550, t1339, t22827, t22833, t6396, t22820, t22826, t22859, t22864, t22868, t26272, t26295, t28085, t28089, t28091, t28093, t28095, t28097);
        let (t28107, t28108, t28111, t28116, t28117, t28118, t28130) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1908(t28083, t28106, t539, t2015, t6460, t3887, t1842, t26337, t22635, t22633, t1825, t26421);
    (t28088, t28100, t28101, t28107, t28108, t28111, t28116, t28117, t28118, t28130)
}
