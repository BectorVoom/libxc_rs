//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta321 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1378;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta321(t11135: f64, t10292: f64, t281: f64, t415: f64, t1114: f64, t2403: f64, t3298: f64, t699: f64, t3301: f64, t3304: f64, t241: f64, t3439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11195, t11203, t11204, t11211, t11213, t11215, t11217, t11219) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1378(t11135, t10292, t281, t415, t1114, t2403, t3298, t699, t3301, t3304, t241, t3439);
    (t11195, t11203, t11204, t11211, t11213, t11215, t11217, t11219)
}
