//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2492/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2492(t21569: f64, t3070: f64, t42488: f64, t10403: f64, t10408: f64, t17156: f64, t18014: f64, t3071: f64, t4338: f64, t4343: f64, t48607: f64, t50324: f64, t5677: f64, t5867: f64, t5909: f64, t62827: f64, t62832: f64, t62836: f64, t62840: f64, t69742: f64, t70241: f64) -> f64 {
    let t70912 = t3070 * t42488 * t21569;
    let t70917 = t50324 * t5909 / 768.0_f64 - t62827 / 81.0_f64 - t62832 / 324.0_f64 - t62836 / 108.0_f64 - 5.0_f64 / 768.0_f64 * t3070 * t10408 * t17156 * t70241 + t3070 * t3071 * t5677 * t70241 / 256.0_f64 - t3070 * t3071 * t5867 * t4343 / 768.0_f64 + t10403 * t3071 * t62840 * t18014 / 768.0_f64 + 5.0_f64 / 4608.0_f64 * t3070 * t10408 * t5867 * t4338 + 5.0_f64 / 6912.0_f64 * t70912 + t48607 * t3071 * t69742 / 256.0_f64;
    t70917
}
