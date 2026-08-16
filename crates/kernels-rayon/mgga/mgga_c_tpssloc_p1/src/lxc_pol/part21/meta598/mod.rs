//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta598 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2349;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2350;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta598(t20: f64, t60: f64, t1799: f64, t3701: f64, t9108: f64, t94: f64, t102: f64, t9174: f64, t2: f64, t591: f64, t21: f64, t9: f64, t14: f64, t2230: f64, t22811: f64, t19: f64, t2233: f64, t2239: f64, t601: f64, t9238: f64, t85: f64, t24: f64, t10276: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t32253, t33159, t35577, t35761, t39031, t39033) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2349(t20, t60, t1799, t3701, t9108, t94, t102, t9174, t2, t591, t21, t9);
        let (t39037, t39043, t39049, t39054, t39063, t39096) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2350(t14, t2230, t22811, t19, t2233, t2239, t601, t9238, t85, t24, t10276, t73);
    (t32253, t33159, t35577, t35761, t39031, t39033, t39037, t39043, t39049, t39054, t39063, t39096)
}
