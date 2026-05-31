//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 432/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk432<F: Float>(t3100: F, t64: F, t1603: F, t1617: F, t1624: F, t1669: F, t3019: F, t3022: F, t3025: F, t3030: F, t3034: F, t3038: F, t3058: F, t3061: F, t3067: F, t3071: F, t3076: F, t3078: F, t372: F, t374: F, t399: F, t940: F) -> F {
    let t3101 = t64 * t3100;
    let t3102 = F::cast_from(0.67598802253579164263e-4_f64) * t3019 * t3022 - F::cast_from(0.23254900946437792e-1_f64) * t1603 * t374 * t3025 - F::cast_from(0.68920324918704953981e-4_f64) * t1617 * t3030 + F::cast_from(0.11627450473218896e-1_f64) * t1624 * t3034 + F::cast_from(0.23254900946437792e-2_f64) * t372 * t3038 - F::cast_from(0.11627450473218896e-1_f64) * t372 * t3058 + F::cast_from(0.19365723406274399941e-3_f64) * t372 * t3061 + F::cast_from(0.11627450473218896e-1_f64) * t1624 * t3067 - F::cast_from(2.0_f64) * t1669 * t3071 - F::cast_from(0.59273806478425129876e-2_f64) * t940 * t399 + F::cast_from(2.0_f64) * t3076 * t3078 - t3101;
    t3102
}
