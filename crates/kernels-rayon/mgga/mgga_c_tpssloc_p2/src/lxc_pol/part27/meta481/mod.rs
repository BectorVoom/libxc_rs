//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta481 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1855;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1856;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta481(t1022: f64, t6768: f64, t1060: f64, t6733: f64, t6743: f64, t6801: f64, t1945: f64, t3040: f64, t3201: f64, t1058: f64, t1920: f64, t1950: f64, t23323: f64, t23327: f64, t23601: f64, t23606: f64, t23610: f64, t23614: f64, t23619: f64, t23621: f64, t23626: f64, t23629: f64, t23633: f64, t23637: f64, t23642: f64, t23644: f64, t23647: f64, t23650: f64, t3180: f64, t3200: f64, t6687: f64, t6797: f64, t6811: f64, t6796: f64, t995: f64, t6802: f64, t614: f64, t6794: f64, t131: f64, t350: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23654, t23657, t23658, t23661, t23662, t23664) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1855(t1022, t6768, t1060, t6733, t6743, t6801, t1945, t3040, t3201, t1058, t1920, t1950, t23323, t23327, t23601, t23606, t23610, t23614, t23619, t23621, t23626, t23629, t23633, t23637, t23642, t23644, t23647, t23650, t3180, t3200, t6687, t6797, t6811);
        let t23665 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1856(t6796, t995);
        let (t23666, t23668, t23669, t23670) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1857(t23665, t6802, t614, t6794, t131, t350);
    (t23654, t23657, t23658, t23661, t23662, t23664, t23665, t23666, t23668, t23669, t23670)
}
