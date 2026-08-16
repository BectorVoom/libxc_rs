//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1756;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1757;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta386(t13380: f64, t829: f64, t252: f64, t4233: f64, t4182: f64, t2684: f64, t4282: f64, t4290: f64, t808: f64, t68: f64, t9971: f64, t226: f64, t13263: f64, t2633: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13381, t13384) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1756(t13380, t829, t252, t4233);
        let (t13385, t13388, t13390, t13393, t13396, t13397, t13398, t13401) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1757(t13384, t4182, t2684, t4282, t4290, t808, t13380, t68, t9971, t226, t13263, t2633);
    (t13381, t13384, t13385, t13388, t13390, t13393, t13396, t13397, t13398, t13401)
}
