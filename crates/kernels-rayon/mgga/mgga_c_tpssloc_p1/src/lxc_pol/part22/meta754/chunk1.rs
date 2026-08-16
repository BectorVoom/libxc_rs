//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2534/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2534(t136: f64, t3297: f64, t71138: f64, t21746: f64, t699: f64, t21750: f64, t50827: f64, t50834: f64, t63291: f64, t63306: f64, t63308: f64, t63841: f64, t63843: f64, t63845: f64) -> (f64, f64, f64, f64) {
    let t71333 = t136 * t3297 * t71138;
    let t71335 = t699 * t21746;
    let t71337 = t699 * t21750;
    let t71343 = -0.60385000000000000002e0_f64 * t63291 + 0.20128333333333333334e0_f64 * t63306 - 0.33547222222222222222e0_f64 * t63308 - 0.27595e-1_f64 * t71333 + 0.5519e-1_f64 * t71335 - 0.33114e0_f64 * t71337 + t50827 - 0.93932222222222222225e0_f64 * t50834 - 0.73586666666666666666e-1_f64 * t63841 - 0.33114e0_f64 * t63843 + 0.5519e-1_f64 * t63845;
    (t71333, t71335, t71337, t71343)
}
