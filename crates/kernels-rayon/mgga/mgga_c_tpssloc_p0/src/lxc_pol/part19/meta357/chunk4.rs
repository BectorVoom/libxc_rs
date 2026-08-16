//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1298/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1298(t41654: f64, t41961: f64, t41937: f64, t41940: f64, t41943: f64, t41945: f64, t41948: f64, t41951: f64, t41954: f64, t41957: f64, t41964: f64, t41967: f64, t41970: f64, t41973: f64) -> f64 {
    let t42212 = 0.5356037037037037037e1_f64 * t41654;
    let t42213 = 0.16979925925925925926e1_f64 * t41961;
    let t42218 = -0.6618234375e1_f64 * t41937 - 0.52945875e1_f64 * t41940 + 0.2366859375e0_f64 * t41943 + 0.94674375e0_f64 * t41945 - 0.705945e1_f64 * t41948 + 0.1262325e1_f64 * t41951 + 0.158837625e2_f64 * t41954 - 0.94674375e0_f64 * t41957 + t42212 + t42213 - 0.13892666666666666667e0_f64 * t41964 - 0.27785333333333333334e0_f64 * t41967 - 0.375102e1_f64 * t41970 + 0.83356e0_f64 * t41973;
    t42218
}
