//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3415/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3415(t51909: f64, t51911: f64, t51913: f64, t51915: f64, t51917: f64, t51921: f64, t51923: f64, t63238: f64, t63240: f64, t63242: f64, t63246: f64, t63250: f64, t63255: f64, t63260: f64) -> f64 {
    let t64197 = -0.55570666666666666668e0_f64 * t51909 + 0.9261777777777777778e-1_f64 * t51911 + 0.9261777777777777778e0_f64 * t51913 - 0.15436296296296296297e0_f64 * t51915 - 0.27785333333333333334e0_f64 * t51917 + 0.4630888888888888889e-1_f64 * t51921 + 0.61745185185185185187e-1_f64 * t51923 - 0.62517e0_f64 * t63238 + 0.83356000000000000001e0_f64 * t63240 - 0.55570666666666666667e0_f64 * t63242 - 0.62517e0_f64 * t63246 + 0.41678e0_f64 * t63250 + 0.41678e0_f64 * t63255 - 0.69463333333333333334e-1_f64 * t63260;
    t64197
}
