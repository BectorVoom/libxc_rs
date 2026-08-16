//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta481 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1855;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1856;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta481<F: Float>(t1022: F, t6768: F, t1060: F, t6733: F, t6743: F, t6801: F, t1945: F, t3040: F, t3201: F, t1058: F, t1920: F, t1950: F, t23323: F, t23327: F, t23601: F, t23606: F, t23610: F, t23614: F, t23619: F, t23621: F, t23626: F, t23629: F, t23633: F, t23637: F, t23642: F, t23644: F, t23647: F, t23650: F, t3180: F, t3200: F, t6687: F, t6797: F, t6811: F, t6796: F, t995: F, t6802: F, t614: F, t6794: F, t131: F, t350: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23654, t23657, t23658, t23661, t23662, t23664) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1855::<F>(t1022, t6768, t1060, t6733, t6743, t6801, t1945, t3040, t3201, t1058, t1920, t1950, t23323, t23327, t23601, t23606, t23610, t23614, t23619, t23621, t23626, t23629, t23633, t23637, t23642, t23644, t23647, t23650, t3180, t3200, t6687, t6797, t6811);
        let t23665 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1856::<F>(t6796, t995);
        let (t23666, t23668, t23669, t23670) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1857::<F>(t23665, t6802, t614, t6794, t131, t350);
    (t23654, t23657, t23658, t23661, t23662, t23664, t23665, t23666, t23668, t23669, t23670)
}
