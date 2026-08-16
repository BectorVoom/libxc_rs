//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1776;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1777;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta436(t25: f64, t6320: f64, t67: f64, t758: f64, t12061: f64, t6305: f64, t3664: f64, t5397: f64, t16557: f64, t2219: f64, t5134: f64, t514: f64, t606: f64, zeta_threshold: f64, t28: f64, t12072: f64, t6312: f64, t3672: f64, t5966: f64, t1081: f64, t18196: f64, t5142: f64, t517: f64, t157: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t19541, t19542, t19543, t19547, t19558) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1776(t25, t6320, t67, t758, t12061, t6305, t3664, t5397, t16557, t2219, t5134, t514, t606, zeta_threshold);
        let (t19559, t19572) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1777(t28, t12072, t6312, t3672, t5966, t1081, t18196, t2219, t5142, t517, t157, t19558, zeta_threshold);
    (t19541, t19542, t19543, t19547, t19559, t19572)
}
