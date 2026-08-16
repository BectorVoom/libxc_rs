//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta446 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1755;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1756;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1757;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1758;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta446(t22882: f64, t6637: f64, t6888: f64, t3719: f64, t6968: f64, t117: f64, t547: f64, t67: f64, t6559: f64, t225: f64, t794: f64, t6969: f64, t3787: f64, t6604: f64, t22740: f64, t3792: f64, t1992: f64, t1336: f64, t2013: f64, t22743: f64, t22746: f64, t22749: f64, t22753: f64, t22871: f64, t22874: f64, t22877: f64, t22879: f64, t3773: f64, t544: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22883, t22884, t22886, t22887, t22888, t22891, t22892) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1755(t22882, t6637, t6888, t3719, t6968, t117, t547, t67, t6559);
        let t22893 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1756(t225, t794);
        let (t22894, t22895, t22896, t22897) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1757(t22893, t6969, t22892, t3787, t6604);
        let (t22898, t22899, t22903) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1758(t22740, t3792, t22897, t1992, t1336, t2013, t22743, t22746, t22749, t22753, t22871, t22874, t22877, t22879, t22884, t22888, t22896, t3773, t544);
    (t22883, t22886, t22887, t22891, t22892, t22893, t22894, t22895, t22897, t22898, t22899, t22903)
}
