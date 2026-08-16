//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1872;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1873;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta509(t25241: f64, t6646: f64, t1888: f64, t23110: f64, t7524: f64, t23185: f64, t234: f64, t6604: f64, t1484: f64, t252: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t25242, t25243, t25245, t25246, t25248) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1872(t25241, t6646, t1888, t23110, t7524, t23185, t234, t6604);
        let t25249 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1873(t1484, t252);
    (t25242, t25243, t25245, t25246, t25248, t25249)
}
