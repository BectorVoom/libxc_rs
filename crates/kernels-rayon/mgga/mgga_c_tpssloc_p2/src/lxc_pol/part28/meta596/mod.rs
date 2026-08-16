//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta596 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1893;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1894;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta596(t2307: f64, t72: f64, t7431: f64, t1437: f64, t6509: f64, t1864: f64, t4021: f64, t1410: f64, t9231: f64, t2240: f64, t3961: f64, t3967: f64, t12571: f64, t608: f64, t33: f64, t46099: f64, t2244: f64, t3953: f64, t9239: f64, t2241: f64, t12648: f64, t605: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90080, t90090, t90094, t90098, t90101, t90104) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1893(t2307, t72, t7431, t1437, t6509, t1864, t4021, t1410, t9231, t2240, t3961, t3967);
        let (t90114, t90121, t90132, t90137, t90141, t90150) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1894(t12571, t608, t33, t46099, t2244, t3953, t1410, t9239, t2241, t72, t7431, t12648, t605);
    (t90080, t90090, t90094, t90098, t90101, t90104, t90114, t90121, t90132, t90137, t90141, t90150)
}
