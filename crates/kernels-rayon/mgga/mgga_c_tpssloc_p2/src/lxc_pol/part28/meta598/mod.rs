//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta598 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1897;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta598(t1410: f64, t9228: f64, t2235: f64, t3961: f64, t3967: f64, t4072: f64, t649: f64, t12813: f64, t88: f64, t1458: f64, t2311: f64, t1845: f64, t3914: f64, t24994: f64, t6875: f64, t26351: f64, t6883: f64, t1992: f64, t26355: f64, t80650: f64, t22635: f64, t26354: f64, t3911: f64, t22751: f64, t26186: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90337, t90340, t90343, t90370, t90375, t90381, t90437) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1897(t1410, t9228, t2235, t3961, t3967, t4072, t649, t12813, t88, t1458, t2311, t1845, t3914);
        let (t90442, t90459, t90462, t90466, t90468) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1898(t24994, t6875, t26351, t6883, t1992, t26355, t80650, t22635, t26354, t3911, t22751, t26186);
    (t90337, t90340, t90343, t90370, t90375, t90381, t90437, t90442, t90459, t90462, t90466, t90468)
}
