//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1220;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1221;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta326(t584: f64, t9212: f64, t111: f64, t4025: f64, t1454: f64, t2281: f64, t4044: f64, t626: f64, t4068: f64, t2341: f64, t92: f64, t100: f64, t2349: f64, t4098: f64, t751: f64, t172: f64, t4095: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12603, t12604, t12725) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1220(t584, t9212, t111, t4025);
        let (t12747, t12750, t12752, t12774, t12795, t12850, t12858) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1221(t1454, t2281, t4044, t626, t4068, t2341, t92, t100, t2349, t4098, t751, t172, t4095);
    (t12603, t12604, t12725, t12747, t12750, t12752, t12774, t12795, t12850, t12858)
}
