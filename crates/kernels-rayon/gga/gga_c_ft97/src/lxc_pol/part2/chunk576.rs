//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 576/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk576(t3817: f64, t695: f64, t1417: f64, t224: f64, t2387: f64, t3723: f64, t3726: f64, t3730: f64, t3734: f64, t3752: f64, t3755: f64, t3759: f64, t3760: f64, t3762: f64, t3766: f64, t3768: f64, t3774: f64, t3777: f64, t3782: f64, t3786: f64, t3789: f64, t3791: f64, t678: f64) -> f64 {
    let t3818 = t695 * t3817;
    let t3820 = 0.67598802253579164263e-4_f64 * t3723 * t3726 + 0.11627450473218896e-1_f64 * t2387 * t3730 + 0.23254900946437792e-2_f64 * t678 * t3734 - 0.11627450473218896e-1_f64 * t678 * t3752 + 0.19365723406274399941e-3_f64 * t678 * t3755 - 0.23254900946437792e-1_f64 * t3759 * t3760 * t3762 - 2.0_f64 * t3766 * t3768 - 0.68920324918704953981e-4_f64 * t3774 * t3777 - 0.59273806478425129876e-2_f64 * t1417 * t3782 + 0.11627450473218896e-1_f64 * t2387 * t3786 + 2.0_f64 * t3789 * t3791 - t224 * t3818;
    t3820
}
