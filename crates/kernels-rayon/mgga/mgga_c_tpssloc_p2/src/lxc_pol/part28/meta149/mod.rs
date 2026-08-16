//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta149 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk777;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk778;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta149(t3236: f64, t407: f64, t3271: f64, t1107: f64, t3279: f64, t281: f64, t2820: f64, t415: f64, t1114: f64, t699: f64, t1176: f64, t241: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3282, t3287, t3288, t3290, t3293, t3294, t3295) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk777(t3236, t407, t3271, t1107, t3279, t281, t2820, t415, t1114, t699);
        let t3297 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk778(t1176, t241);
    (t3282, t3287, t3288, t3290, t3293, t3294, t3295, t3297)
}
