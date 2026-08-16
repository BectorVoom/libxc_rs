//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta434 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1755;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1756;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta434(t22779: f64, t6937: f64, t6950: f64, t835: f64, t1336: f64, t1369: f64, t3876: f64, t6952: f64, t3777: f64, t6951: f64, t6597: f64, t6924: f64, t281: f64, t1307: f64, t1361: f64, t22690: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22780, t22782, t22783) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1755(t22779, t6937, t6950, t835, t1336);
        let (t22784, t22785, t22786, t22788, t22789, t22791, t22792, t22794) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1756(t1369, t22783, t3876, t6952, t3777, t6951, t6597, t6924, t281, t1307, t1361, t22690);
    (t22780, t22782, t22783, t22784, t22785, t22786, t22788, t22789, t22791, t22792, t22794)
}
