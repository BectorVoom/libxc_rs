//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1289/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1289<F: Float>(t112860: F, t113843: F, t113922: F, t114089: F, t114177: F, t114182: F, t114214: F, t114752: F, t25427: F, t25452: F, t2649: F, t2745: F, t29008: F, t4135: F, t6391: F, t6963: F, t7129: F, t99975: F) -> (F,) {
    let t115046 = 4.0 * t114089 + 8.0 * t112860 - 4.0 * t114752 + t6963 * t25427 / 6.0 - t99975 / 18.0 - t2745 * t7129 - t2649 * t7129 - 2.0 * t113843 - 2.0 * t4135 * t6391 - 4.0 * t114182 - 2.0 * t114214 - 2.0 * t114177 - t29008 * t25452 / 18.0 - 2.0 * t113922;
    (t115046,)
}
