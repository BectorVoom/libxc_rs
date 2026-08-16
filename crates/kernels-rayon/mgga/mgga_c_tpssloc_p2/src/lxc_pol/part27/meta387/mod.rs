//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta387 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1588;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1589;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta387(t14704: f64, t1089: f64, t12606: f64, t1088: f64, t123: f64, t4778: f64, t699: f64, t1113: f64, t136: f64, t4725: f64, t690: f64, t4730: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14705, t14706, t14708) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1588(t14704, t1089, t12606, t1088, t123);
        let (t14710, t14711, t14713, t14720) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1589(t4778, t699, t1113, t14706, t136, t4725, t690);
        let (t14721, t14722) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1590(t14720, t4730, t690);
    (t14705, t14706, t14708, t14710, t14711, t14713, t14720, t14721, t14722)
}
