//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta132 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk705;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk706;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta132(t1203: f64, t1222: f64, t221: f64, t3426: f64, t456: f64, t1197: f64, t135: f64, t1174: f64, t1176: f64, t3247: f64, t3242: f64, t3439: f64, t121: f64, t486: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3543, t3545, t3547, t3548, t3549, t3555, t3560) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk705(t1203, t1222, t221, t3426, t456, t1197, t135, t1174, t1176, t3247, t3242, t3439);
        let t3570 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk706(t121, t486);
    (t3543, t3545, t3547, t3548, t3549, t3555, t3560, t3570)
}
