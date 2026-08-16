//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2345/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2345(t13012: f64, t20927: f64, t13005: f64, t41144: f64, t41155: f64, t41156: f64, t41185: f64, t41190: f64, t46764: f64, t46769: f64, t46838: f64, t59138: f64, t59140: f64, t68010: f64) -> f64 {
    let t68073 = t13012 * t20927;
    let t68077 = -0.59999999999999999996e-1_f64 * t13005 * t46838 * t68010 - 0.19999999999999999999e-1_f64 * t41144 + t41155 + 0.56172839506172839504e-1_f64 * t41156 - t41185 + 0.3287037037037037037e-1_f64 * t41190 - 0.59999999999999999998e-1_f64 * t46764 + t46769 - 0.34999999999999999998e-1_f64 * t68073 - 0.74999999999999999997e-2_f64 * t59138 - 0.34999999999999999998e-1_f64 * t59140;
    t68077
}
