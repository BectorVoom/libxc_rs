//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta100 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk619;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk620;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta100(t2319: f64, t89: f64, t1266: f64, t671: f64, t107: f64, t2281: f64, t626: f64, t667: f64, t106: f64, t655: f64, t666: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t2320, t2323, t2327, t2328, t2331) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk619(t2319, t89, t1266, t671, t107, t2281, t626, t667, t106, t655);
        let t2332 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk620(t666);
    (t2320, t2323, t2327, t2328, t2331, t2332)
}
