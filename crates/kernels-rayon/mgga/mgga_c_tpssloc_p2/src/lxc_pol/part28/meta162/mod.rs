//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta162 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk810;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta162(t374: f64, t486: f64, t677: f64, t485: f64, t1203: f64, t1222: f64, t221: f64, t3426: f64, t456: f64, t1197: f64, t135: f64, t1174: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t3540, t3542, t3543, t3545, t3547, t3548, t3549) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk810(t374, t486, t677, t485, t1203, t1222, t221, t3426, t456, t1197, t135, t1174);
    (t3540, t3542, t3543, t3545, t3547, t3548, t3549)
}
