//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta448 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1711;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta448(t1369: f64, t22783: f64, t3777: f64, t6951: f64, t6597: f64, t6924: f64, t281: f64, t1307: f64, t1361: f64, t22690: f64, t547: f64, t6546: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t22784, t22788, t22791, t22792, t22794, t22795, t22797) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1711(t1369, t22783, t3777, t6951, t6597, t6924, t281, t1307, t1361, t22690, t547, t6546);
    (t22784, t22788, t22791, t22792, t22794, t22795, t22797)
}
