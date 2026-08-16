//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 592/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk592(t449: f64, t8232: f64, t1868: f64, t1882: f64, t110: f64, t1866: f64, t7959: f64, t7748: f64, t7758: f64, t7768: f64, t7775: f64, t7778: f64, t7791: f64, t7796: f64, t7809: f64, t7813: f64, t7817: f64, t7822: f64, t7827: f64, t7831: f64) -> (f64, f64, f64, f64) {
    let t8233 = t8232 * t449;
    let t8235 = t1882 * t1868;
    let t8238 = t1866 * t110 * t7959;
    let t8252 = 2.0_f64 * t7791 + 2.0_f64 / 3.0_f64 * t7796 - 2.0_f64 / 3.0_f64 * t7809 + t7813 + t7817 - 2.0_f64 / 3.0_f64 * t7822 - 2.0_f64 * t7827 - 2.0_f64 * t7831 - t7748 / 3.0_f64 + 6.0_f64 * t7758 - 10.0_f64 / 27.0_f64 * t7768 - 4.0_f64 / 9.0_f64 * t7775 + t7778 / 3.0_f64;
    (t8233, t8235, t8238, t8252)
}
