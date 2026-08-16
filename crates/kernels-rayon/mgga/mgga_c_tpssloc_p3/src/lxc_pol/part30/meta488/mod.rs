//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1792;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1793;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta488(t1484: f64, t252: f64, t776: f64, t25248: f64, t25038: f64, t7510: f64, t814: f64, t829: f64, t7528: f64, t794: f64, t6562: f64, t1509: f64, t1902: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t25249 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1792(t1484, t252);
        let (t25250, t25251, t25252, t25255, t25256, t25258, t25259, t25261) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1793(t25249, t776, t25248, t25038, t7510, t814, t829, t7528, t794, t6562, t1509, t1902);
    (t25249, t25250, t25251, t25252, t25255, t25256, t25258, t25259, t25261)
}
