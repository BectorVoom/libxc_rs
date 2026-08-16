//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1800;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1801;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta539(t81763: f64, t849: f64, t6620: f64, t9612: f64, t23132: f64, t2617: f64, t23133: f64, t2707: f64, t131: f64, t23121: f64, t9537: f64, t236: f64, t81613: f64, t23098: f64, t22822: f64, t281: f64, t6589: f64, t23124: f64, t23076: f64, t6597: f64, t22690: f64, t2379: f64, t841: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81764, t81766, t81769, t81770, t81772, t81782, t81783) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1800(t81763, t849, t6620, t9612, t23132, t2617, t23133, t2707, t131, t23121, t9537, t236, t81613);
        let (t81785, t81788, t81789, t81792, t81795) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1801(t23098, t81782, t81783, t22822, t281, t6589, t23124, t23076, t6597, t22690, t2379, t841);
    (t81764, t81766, t81769, t81770, t81772, t81782, t81783, t81785, t81788, t81789, t81792, t81795)
}
