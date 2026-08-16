//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1614;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1615;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta435(t23121: f64, t281: f64, t22690: f64, t776: f64, t841: f64, t2617: f64, t6620: f64, t849: f64, t2703: f64, t6621: f64, t6619: f64, t835: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23122, t23124, t23125, t23127, t23128, t23130, t23132) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1614(t23121, t281, t22690, t776, t841, t2617, t6620, t849, t2703, t6621, t6619, t835);
        let t23133 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1615(t23132, t812);
    (t23122, t23124, t23125, t23127, t23128, t23130, t23132, t23133)
}
