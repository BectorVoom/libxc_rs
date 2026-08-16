//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1254;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1255;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta345(t4734: f64, t690: f64, t4778: f64, t699: f64, t4725: f64, t4730: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t14704, t14705, t14710, t14711, t14720) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1254(t4734, t690, t4778, t699, t4725);
        let (t14721, t14722) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1255(t14720, t4730, t690);
    (t14704, t14705, t14710, t14711, t14720, t14721, t14722)
}
