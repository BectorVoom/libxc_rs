//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 327/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk327<F: Float>(t3099: F, t409: F, t64: F, t1603: F, t1617: F, t1624: F, t1669: F, t3019: F, t3022: F, t3025: F, t3030: F, t3034: F, t3038: F, t3058: F, t3061: F, t3067: F, t3071: F, t3076: F, t3078: F, t372: F, t374: F, t399: F, t940: F) -> (F,) {
    let t3100 = t409 * t3099;
    let t3101 = t64 * t3100;
    let t3102 = 0.67598802253579164263e-4 * t3019 * t3022 - 0.23254900946437792e-1 * t1603 * t374 * t3025 - 0.68920324918704953981e-4 * t1617 * t3030 + 0.11627450473218896e-1 * t1624 * t3034 + 0.23254900946437792e-2 * t372 * t3038 - 0.11627450473218896e-1 * t372 * t3058 + 0.19365723406274399941e-3 * t372 * t3061 + 0.11627450473218896e-1 * t1624 * t3067 - 2.0 * t1669 * t3071 - 0.59273806478425129876e-2 * t940 * t399 + 2.0 * t3076 * t3078 - t3101;
    (t3102,)
}
