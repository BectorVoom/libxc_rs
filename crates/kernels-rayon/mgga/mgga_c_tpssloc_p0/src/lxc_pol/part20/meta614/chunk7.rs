//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2211/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2211(t12677: f64, t12681: f64, t12684: f64, t12687: f64, t1414: f64, t1420: f64, t2262: f64, t39: f64, t39210: f64, t3982: f64, t3985: f64, t43: f64, t45872: f64, t51: f64, t55: f64, t615: f64, t9277: f64, t9301: f64, t9308: f64) -> f64 {
    let t45931 = -3080.0_f64 / 81.0_f64 * t9277 * t1414 + 220.0_f64 / 9.0_f64 * t2262 * t3985 - 20.0_f64 / 3.0_f64 * t615 * t12687 + 5.0_f64 / 6.0_f64 * t39 * t43 * t45872 - 10.0_f64 / 81.0_f64 * t1420 * t9301 + 20.0_f64 / 9.0_f64 * t1420 * t9308 - 5.0_f64 / 6.0_f64 * t51 * t55 * t45872 - t39210 + 220.0_f64 / 27.0_f64 * t2262 * t3982 - 40.0_f64 / 9.0_f64 * t615 * t12681 - 20.0_f64 / 9.0_f64 * t615 * t12684 + 10.0_f64 / 27.0_f64 * t615 * t12677;
    t45931
}
