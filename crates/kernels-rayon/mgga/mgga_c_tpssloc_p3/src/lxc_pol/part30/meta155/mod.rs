//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta155 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk814;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk815;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta155(t2645: f64, t2647: f64, t4181: f64, t157: f64, t2658: f64, t1409: f64, t184: f64, t607: f64, t1474: f64, t172: f64, t763: f64, t185: f64, t3966: f64, t707: f64, t1471: f64, t706: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4191 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk814(t2645, t2647, t4181);
        let (t4194, t4195, t4196, t4198, t4199, t4200, t4201, t4202, t4204, t4205) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk815(t157, t2658, t1409, t184, t607, t1474, t172, t763, t185, t3966, t707, t1471, t706);
    (t4191, t4194, t4195, t4196, t4198, t4199, t4200, t4201, t4202, t4204, t4205)
}
