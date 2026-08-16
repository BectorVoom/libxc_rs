//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta156 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk770;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta156(t31: f64, t9258: f64, t65: f64, t2251: f64, t628: f64, t2283: f64, t608: f64, t36: f64, t366: f64, t41: f64, t42: f64, t2244: f64, t607: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9259, t9260, t9263, t9268, t9276, t9277, t9287, t9288) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk770(t31, t9258, t65, t2251, t628, t2283, t608, t36, t366, t41, t42, t2244, t607, sigma0);
    (t9259, t9260, t9263, t9268, t9276, t9277, t9287, t9288)
}
