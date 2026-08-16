//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta598 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1897;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta598<F: Float>(t1410: F, t9228: F, t2235: F, t3961: F, t3967: F, t4072: F, t649: F, t12813: F, t88: F, t1458: F, t2311: F, t1845: F, t3914: F, t24994: F, t6875: F, t26351: F, t6883: F, t1992: F, t26355: F, t80650: F, t22635: F, t26354: F, t3911: F, t22751: F, t26186: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t90337, t90340, t90343, t90370, t90375, t90381, t90437) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1897::<F>(t1410, t9228, t2235, t3961, t3967, t4072, t649, t12813, t88, t1458, t2311, t1845, t3914);
        let (t90442, t90459, t90462, t90466, t90468) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1898::<F>(t24994, t6875, t26351, t6883, t1992, t26355, t80650, t22635, t26354, t3911, t22751, t26186);
    (t90337, t90340, t90343, t90370, t90375, t90381, t90437, t90442, t90459, t90462, t90466, t90468)
}
