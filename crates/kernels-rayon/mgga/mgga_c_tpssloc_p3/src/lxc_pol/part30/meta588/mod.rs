//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1967;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta588(t5544: f64, t868: f64, t5527: f64, t1484: f64, t4303: f64, t4233: f64, t828: f64, t1388: f64, t6347: f64, t1799: f64, t5356: f64, t1351: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t67123, t67128, t67164, t67783, t67793, t74032, t74060, t74366) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1967(t5544, t868, t5527, t1484, t4303, t4233, t828, t1388, t6347, t1799, t5356, t1351);
    (t67123, t67128, t67164, t67783, t67793, t74032, t74060, t74366)
}
