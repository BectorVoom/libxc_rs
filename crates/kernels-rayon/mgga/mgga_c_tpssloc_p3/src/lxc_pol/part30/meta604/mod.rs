//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1994;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta604(t81849: f64, t1887: f64, t206: f64, t80845: f64, t23145: f64, t2617: f64, t23102: f64, t80782: f64, t23113: f64, t23093: f64, t281: f64, t23046: f64, t812: f64, t835: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81850, t81853, t81865, t81876, t81877, t81882, t81883, t81886) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1994(t81849, t1887, t206, t80845, t23145, t2617, t23102, t80782, t23113, t23093, t281, t23046, t812, t835);
    (t81850, t81853, t81865, t81876, t81877, t81882, t81883, t81886)
}
