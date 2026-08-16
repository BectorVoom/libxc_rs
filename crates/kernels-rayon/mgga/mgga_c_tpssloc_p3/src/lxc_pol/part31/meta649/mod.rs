//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta649 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1924;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1925;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta649(t16759: f64, t1888: f64, t6646: f64, t17030: f64, t22986: f64, t2647: f64, t17046: f64, t1510: f64, t87130: f64, t25249: f64, t4234: f64, t23110: f64, t28337: f64, t81651: f64, t87111: f64, t16820: f64, t22996: f64, t17031: f64, t829: f64, t98389: f64, t16815: f64, t9627: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98428, t98432, t98435, t98439, t98443, t98446) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1924(t16759, t1888, t6646, t17030, t22986, t2647, t17046, t1510, t87130, t25249, t4234, t23110, t28337, t81651);
        let (t98461, t98464, t98467, t98471, t98475) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1925(t1510, t22986, t6646, t87111, t16820, t1888, t22996, t17031, t829, t98389, t16815, t9627);
    (t98428, t98432, t98435, t98439, t98443, t98446, t98461, t98464, t98467, t98471, t98475)
}
