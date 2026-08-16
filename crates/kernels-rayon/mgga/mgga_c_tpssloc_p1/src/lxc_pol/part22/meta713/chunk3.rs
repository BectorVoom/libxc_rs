//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2315/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2315(t67262: f64, t67280: f64, t12895: f64, t193: f64, t20756: f64, t2522: f64, t39549: f64, t39563: f64, t4314: f64, t5527: f64, t67226: f64, t67228: f64, t67231: f64, t67235: f64, t67239: f64, t67244: f64, t766: f64, t776: f64, t868: f64, t870: f64) -> (f64, f64) {
    let t67282 = t67262 / 2.0_f64 + t67280 / 2.0_f64;
    let t67286 = 6.0_f64 * t193 * t20756 * t868 * t870 + 18.0_f64 * t12895 * t4314 * t5527 + 3.0_f64 * t193 * t67282 * t766 + 6.0_f64 * t2522 * t67239 * t776 + 6.0_f64 * t4314 * t67235 * t776 + t39549 + t39563 + t67226 + t67228 + t67231 + t67244;
    (t67282, t67286)
}
