//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1285/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1285(t41937: f64, t41940: f64, t41943: f64, t41945: f64, t41948: f64, t41951: f64, t41954: f64, t41957: f64, t41959: f64, t41962: f64, t41964: f64, t41967: f64, t41970: f64, t41973: f64) -> f64 {
    let t41975 = -0.485484375e1_f64 * t41937 - 0.3883875e1_f64 * t41940 + 0.6189328125e-1_f64 * t41943 + 0.247573125e0_f64 * t41945 - 0.51785e1_f64 * t41948 + 0.3300975e0_f64 * t41951 + 0.11651625e2_f64 * t41954 - 0.247573125e0_f64 * t41957 + t41959 + t41962 - 0.11038e0_f64 * t41964 - 0.22076e0_f64 * t41967 - 0.298026e1_f64 * t41970 + 0.66228e0_f64 * t41973;
    t41975
}
