//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta529(t21: f64, t9: f64, t587: f64, t598: f64, t14: f64, t2230: f64, t594: f64, t9223: f64, t22811: f64, t19: f64, t601: f64, t9238: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t39033, t39035, t39037, t39039, t39043, t39054) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2000(t21, t9, t587, t598, t14, t2230, t594, t9223, t22811, t19, t601, t9238);
    (t39033, t39035, t39037, t39039, t39043, t39054)
}
