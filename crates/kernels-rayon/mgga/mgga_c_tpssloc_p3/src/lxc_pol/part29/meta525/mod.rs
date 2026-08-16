//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta525(t1388: f64, t1845: f64, t26162: f64, t26161: f64, t532: f64, t7752: f64, t6879: f64, t1983: f64, t1874: f64, t26114: f64, t4072: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26163, t26164, t26166, t26167, t26168, t26170, t26178, t26179) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1904(t1388, t1845, t26162, t26161, t532, t7752, t6879, t1983, t1874, t26114, t4072, t89);
    (t26163, t26164, t26166, t26167, t26168, t26170, t26178, t26179)
}
