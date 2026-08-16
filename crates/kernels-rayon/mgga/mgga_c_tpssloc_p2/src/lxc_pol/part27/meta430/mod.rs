//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta430 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1749;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta430(t22724: f64, t6973: f64, t6982: f64, t794: f64, t6897: f64, t6883: f64, t6983: f64, t1307: f64, t562: f64, t1352: f64, t6976: f64, t22633: f64, t1332: f64, t1336: f64, t22693: f64, t22697: f64, t22701: f64, t22707: f64, t22710: f64, t22718: f64, t22721: f64, t3777: f64, t6988: f64, t6990: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t22726, t22727, t22728, t22730, t22731, t22733, t22734, t22735) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1749(t22724, t6973, t6982, t794, t6897, t6883, t6983, t1307, t562, t1352, t6976, t22633);
        let t22739 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1750(t1332, t1336, t22693, t22697, t22701, t22707, t22710, t22718, t22721, t22726, t22728, t22731, t22735, t3777, t6988, t6990);
    (t22726, t22727, t22728, t22730, t22733, t22734, t22739)
}
