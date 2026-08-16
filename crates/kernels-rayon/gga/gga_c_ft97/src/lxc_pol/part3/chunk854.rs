//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 854/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk854(t16719: f64, t9192: f64, t15742: f64, t3499: f64, t12809: f64, t12816: f64, t12834: f64, t12836: f64, t12839: f64, t12850: f64, t17256: f64, t17261: f64, t17265: f64, t17268: f64, t17272: f64, t17274: f64, t17276: f64, t17279: f64, t17281: f64, t17284: f64, t17286: f64, t3139: f64, t462: f64, t9179: f64, t92: f64) -> f64 {
    let t17289 = t9192 * t16719;
    let t17292 = t3499 * t15742;
    let t17295 = 2.0_f64 * t462 * t17256 - 6.0_f64 * t462 * t17261 + 4.0_f64 * t462 * t17265 - t462 * t17268 / 3.0_f64 - 8.0_f64 / 9.0_f64 * t12809 + t12816 - 2.0_f64 / 9.0_f64 * t17272 + t17274 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t17276 - t12834 - t12836 + t12839 - t12850 - 4.0_f64 / 9.0_f64 * t9179 + t17279 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t17281 - t92 * t17284 + 4.0_f64 / 3.0_f64 * t3139 * t17286 + 2.0_f64 / 9.0_f64 * t462 * t17289 + 4.0_f64 / 3.0_f64 * t462 * t17292;
    t17295
}
