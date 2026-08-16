//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1833;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1834;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta471(t6712: f64, t995: f64, t1941: f64, t3077: f64, t1942: f64, t3082: f64, t344: f64, t40: f64, t1009: f64, t6740: f64, t1015: f64, t6746: f64, t984: f64, t1933: f64, t225: f64, t343: f64, t364: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23463, t23465, t23469, t23470, t23471, t23472, t23473) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1833(t6712, t995, t1941, t3077, t1942, t3082, t344, t40, t1009, t6740, t1015, t6746);
        let (t23474, t23476, t23477, t23478, t23479) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1834(t23472, t23473, t40, t984, t1933, t225, t343, t364);
    (t23463, t23465, t23469, t23470, t23471, t23472, t23473, t23474, t23476, t23477, t23478, t23479)
}
