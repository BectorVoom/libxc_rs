//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1922;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta561(t1527: f64, t7537: f64, t2718: f64, t1911: f64, t5636: f64, t10110: f64, t5657: f64, t16815: f64, t232: f64, t6646: f64, t1888: f64, t5544: f64, t6638: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t28307, t28311, t28317, t28321, t28322, t28323, t28329) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1922(t1527, t7537, t2718, t1911, t5636, t10110, t5657, t16815, t232, t6646, t1888, t5544, t6638);
    (t28307, t28311, t28317, t28321, t28322, t28323, t28329)
}
