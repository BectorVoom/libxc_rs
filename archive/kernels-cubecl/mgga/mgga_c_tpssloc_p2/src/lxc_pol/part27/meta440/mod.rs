//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta440 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1768;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1769;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1770;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1771;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta440<F: Float>(t22882: F, t6637: F, t6888: F, t3719: F, t6968: F, t117: F, t547: F, t67: F, t6559: F, t225: F, t794: F, t6969: F, t3787: F, t6604: F, t22740: F, t3792: F, t1992: F, t1336: F, t2013: F, t22743: F, t22746: F, t22749: F, t22753: F, t22871: F, t22874: F, t22877: F, t22879: F, t3773: F, t544: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22883, t22884, t22886, t22887, t22888, t22891, t22892) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1768::<F>(t22882, t6637, t6888, t3719, t6968, t117, t547, t67, t6559);
        let t22893 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1769::<F>(t225, t794);
        let (t22894, t22895, t22896, t22897) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1770::<F>(t22893, t6969, t22892, t3787, t6604);
        let (t22898, t22899, t22903) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1771::<F>(t22740, t3792, t22897, t1992, t1336, t2013, t22743, t22746, t22749, t22753, t22871, t22874, t22877, t22879, t22884, t22888, t22896, t3773, t544);
    (t22883, t22886, t22887, t22891, t22892, t22893, t22894, t22895, t22897, t22898, t22899, t22903)
}
