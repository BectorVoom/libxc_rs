//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1877;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1878;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta622(t1825: f64, t22633: f64, t6976: f64, t90818: f64, t26421: f64, t5287: f64, t22751: f64, t28149: f64, t19740: f64, t1992: f64, t22897: f64, t28139: f64, t28159: f64, t6897: f64, t794: f64, t19763: f64, t19739: f64, t3807: f64, t28131: f64, t81159: f64, t552: f64, t6434: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97087, t97091, t97095, t97106, t97108) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1877(t1825, t22633, t6976, t90818, t26421, t5287, t22751, t28149, t19740, t1992, t22897, t28139);
        let (t97111, t97114, t97119, t97124, t97126) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1878(t28159, t6897, t794, t19763, t1992, t6976, t19739, t22633, t3807, t28131, t81159, t552, t6434);
    (t97087, t97091, t97095, t97106, t97108, t97111, t97114, t97119, t97124, t97126)
}
