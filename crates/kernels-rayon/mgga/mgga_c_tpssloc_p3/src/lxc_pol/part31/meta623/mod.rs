//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1879;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1880;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta623(t1307: f64, t6637: f64, t6888: f64, t97126: f64, t26331: f64, t26446: f64, t96964: f64, t28164: f64, t6914: f64, t22704: f64, t22705: f64, t28181: f64, t19889: f64, t91004: f64, t91006: f64, t28182: f64, t19660: f64, t22633: f64, t3807: f64, t6976: f64, t22685: f64, t22881: f64, t6330: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97129, t97135, t97137, t97142) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1879(t1307, t6637, t6888, t97126, t26331, t26446, t96964, t28164, t6914, t22704, t22705, t28181);
        let (t97146, t97148, t97152, t97158) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1880(t19889, t91004, t91006, t28182, t6914, t19660, t22633, t3807, t6976, t22685, t22881, t6330, t6637);
    (t97129, t97135, t97137, t97142, t97146, t97148, t97152, t97158)
}
