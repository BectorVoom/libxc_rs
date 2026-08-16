//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta552(t111: f64, t7222: f64, t81437: f64, t22550: f64, t7031: f64, t39054: f64, t7025: f64, t23966: f64, t9231: f64, t39063: f64, t9239: f64, t1860: f64, t23992: f64, t6509: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t84033, t84036, t84173, t84190, t84195, t84216, t84219, t84229) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1780(t111, t7222, t81437, t22550, t7031, t39054, t7025, t23966, t9231, t39063, t9239, t1860, t23992, t6509);
    (t84033, t84036, t84173, t84190, t84195, t84216, t84219, t84229)
}
