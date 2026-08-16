//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta374 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1430;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1431;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1432;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta374<F: Float>(t4778: F, t699: F, t1113: F, t14706: F, t136: F, t4725: F, t690: F, t4730: F, t14704: F, t11147: F, t1409: F, t2244: F, t11145: F, t123: F, t11153: F, t3240: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14710, t14711, t14713, t14720) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1430::<F>(t4778, t699, t1113, t14706, t136, t4725, t690);
        let (t14721, t14722) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1431::<F>(t14720, t4730, t690);
        let (t14723, t14724, t14726, t14728) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1432::<F>(t14722, t14704, t11147, t1409, t2244, t11145, t123);
        let (t14731, t14733) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1433::<F>(t11153, t1409, t2244, t3240, t123);
    (t14710, t14711, t14713, t14720, t14721, t14722, t14723, t14724, t14726, t14728, t14731, t14733)
}
