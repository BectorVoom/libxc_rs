//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta607 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1915;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta607(t22892: f64, t22893: f64, t26384: f64, t16018: f64, t6637: f64, t6888: f64, t6968: f64, t26388: f64, t7733: f64, t81186: f64, t5318: f64, t552: f64, t1307: f64, t1352: f64, t22633: f64, t6976: f64, t90754: f64, t5187: f64, t562: f64, t1799: f64, t81129: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90797, t90801, t90805, t90807, t90809) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1915(t22892, t22893, t26384, t16018, t6637, t6888, t6968, t26388, t7733, t81186, t5318, t552);
        let (t90812, t90816, t90818, t90821, t90825) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1916(t1307, t6637, t6888, t90809, t1352, t22633, t6976, t90754, t5187, t562, t1799, t81129);
    (t90797, t90801, t90805, t90807, t90812, t90816, t90818, t90821, t90825)
}
