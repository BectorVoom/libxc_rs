//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2087/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2087(t10041: f64, t2563: f64, t2678: f64, t776: f64, t222: f64, t39934: f64, t2617: f64, t9637: f64, t2649: f64, t2691: f64, t812: f64, t815: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41088 = t2563 * t10041;
    let t41090 = t776 * t2678;
    let t41096 = 455.0_f64 / 243.0_f64 * t39934 * t222;
    let t41107 = t2617 * t9637;
    let t41108 = t41107 * t2649;
    let t41115 = t812 * t815 * t2691;
    (t41088, t41090, t41096, t41107, t41108, t41115)
}
