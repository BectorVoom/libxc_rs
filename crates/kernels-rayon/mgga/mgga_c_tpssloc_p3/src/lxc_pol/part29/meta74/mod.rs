//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta74 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk503;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk504;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta74(t40: f64, t52: f64, t1409: f64, t185: f64, t707: f64, t73: f64, t76: f64, t145: f64, t157: f64, t182: f64, t767: f64, t771: f64, zeta_threshold: f64, t210: f64, t214: f64, t785: f64, t787: f64, t797: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1462, t1464, t1471, t1472, t1473, t1474, t1476, t1484) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk503(t40, t52, t1409, t185, t707, t73, t76, t145, t157, t182, t767, t771, zeta_threshold);
        let (t1489, t1492) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk504(t1484, t210, t214, t785, t787, t797);
    (t1462, t1464, t1471, t1472, t1473, t1474, t1476, t1484, t1489, t1492)
}
