//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta587 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1879;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1880;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta587(t22986: f64, t6646: f64, t829: f64, t87111: f64, t25273: f64, t6579: f64, t244: f64, t268: f64, t6559: f64, t25250: f64, t87202: f64, t25316: f64, t82038: f64, t1888: f64, t232: f64, t47439: f64, t23110: f64, t23185: f64, t25272: f64, t25325: f64, t6547: f64, t1880: f64, t7488: f64, t82124: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87705, t87709, t87712, t87714, t87718) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1879(t22986, t6646, t829, t87111, t25273, t6579, t244, t268, t6559, t25250, t87202, t25316, t82038);
        let (t87726, t87729, t87733, t87746) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1880(t1888, t232, t47439, t6646, t23110, t23185, t25272, t25325, t6547, t1880, t7488, t82124);
    (t87705, t87709, t87712, t87714, t87718, t87726, t87729, t87733, t87746)
}
