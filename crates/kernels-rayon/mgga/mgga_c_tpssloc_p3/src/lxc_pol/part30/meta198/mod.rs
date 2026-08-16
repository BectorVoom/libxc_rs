//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta198 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk935;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta198(t1323: f64, t1834: f64, t1811: f64, t3726: f64, t1307: f64, t1810: f64, t210: f64, t119: f64, t5187: f64, t225: f64, t5210: f64, t554: f64, t1814: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t5217, t5220, t5223, t5227, t5230) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk935(t1323, t1834, t1811, t3726, t1307, t1810, t210, t119, t5187, t225, t5210);
        let (t5231, t5234) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk936(t5230, t554, t1814, t68);
    (t5217, t5220, t5223, t5227, t5230, t5231, t5234)
}
