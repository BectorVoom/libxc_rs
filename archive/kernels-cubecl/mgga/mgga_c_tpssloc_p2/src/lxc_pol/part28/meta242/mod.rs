//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta242 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1059;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1060;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1061;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta242<F: Float>(t1891: F, t6597: F, t133: F, t119: F, t212: F, t1895: F, t213: F, t225: F, t1892: F, t815: F, t829: F, t1898: F, t808: F, t249: F, t59: F, t814: F, t240: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6598, t6600, t6601, t6602, t6604) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1059::<F>(t1891, t6597, t133, t119, t212, t1895, t213, t225);
        let t6605 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1060::<F>(t1892, t6604);
        let (t6606, t6607, t6609, t6610, t6612) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1061::<F>(t815, t829, t6605, t1898, t808, t249, t59, t814);
        let t6613 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1062::<F>(t240, t6612);
    (t6598, t6600, t6601, t6602, t6604, t6605, t6606, t6607, t6609, t6610, t6612, t6613)
}
