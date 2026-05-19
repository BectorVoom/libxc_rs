//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 576/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk576<F: Float>(t3817: F, t695: F, t1417: F, t224: F, t2387: F, t3723: F, t3726: F, t3730: F, t3734: F, t3752: F, t3755: F, t3759: F, t3760: F, t3762: F, t3766: F, t3768: F, t3774: F, t3777: F, t3782: F, t3786: F, t3789: F, t3791: F, t678: F) -> F {
    let t3818 = t695 * t3817;
    let t3820 = F::cast_from(0.67598802253579164263e-4_f64) * t3723 * t3726 + F::cast_from(0.11627450473218896e-1_f64) * t2387 * t3730 + F::cast_from(0.23254900946437792e-2_f64) * t678 * t3734 - F::cast_from(0.11627450473218896e-1_f64) * t678 * t3752 + F::cast_from(0.19365723406274399941e-3_f64) * t678 * t3755 - F::cast_from(0.23254900946437792e-1_f64) * t3759 * t3760 * t3762 - F::new(2.0) * t3766 * t3768 - F::cast_from(0.68920324918704953981e-4_f64) * t3774 * t3777 - F::cast_from(0.59273806478425129876e-2_f64) * t1417 * t3782 + F::cast_from(0.11627450473218896e-1_f64) * t2387 * t3786 + F::new(2.0) * t3789 * t3791 - t224 * t3818;
    t3820
}
