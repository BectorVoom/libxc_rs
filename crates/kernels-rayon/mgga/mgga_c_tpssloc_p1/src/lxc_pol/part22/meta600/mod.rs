//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2122;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta600(t1580: f64, t2930: f64, t2885: f64, t4408: f64, t47705: f64, t47707: f64, t47730: f64, t10632: f64, t4471: f64, t48096: f64, t2904: f64, t4446: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48783, t48789, t48799, t48800, t48809, t48890, t48919, t48924, t48946, t48947, t48956, t49096) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2122(t1580, t2930, t2885, t4408, t47705, t47707, t47730, t10632, t4471, t48096, t2904, t4446);
    (t48783, t48789, t48799, t48800, t48809, t48890, t48919, t48924, t48946, t48947, t48956, t49096)
}
