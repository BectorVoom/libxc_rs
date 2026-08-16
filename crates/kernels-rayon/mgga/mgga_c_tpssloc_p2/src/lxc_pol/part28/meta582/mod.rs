//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1869;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1870;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta582(t232: f64, t46693: f64, t6605: f64, t815: f64, t2628: f64, t58345: f64, t2632: f64, t47262: f64, t22996: f64, t6590: f64, t25130: f64, t828: f64, t9627: f64, t22986: f64, t25249: f64, t2679: f64, t6646: f64, t23110: f64, t25299: f64, t81651: f64, t23168: f64, t25313: f64, t25319: f64, t2553: f64, t6552: f64, t6637: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87495, t87498, t87502, t87507) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1869(t232, t46693, t6605, t815, t2628, t58345, t2632, t47262, t22996, t6590, t25130, t828, t9627);
        let (t87517, t87520, t87522, t87527) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1870(t22986, t25249, t2679, t6646, t23110, t25299, t81651, t23168, t25313, t25319, t2553, t6552, t6637);
    (t87495, t87498, t87502, t87507, t87517, t87520, t87522, t87527)
}
