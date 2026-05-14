//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1156/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1156<F: Float>(t116436: F, t29706: F, t379: F, t22958: F, t5674: F, t102114: F, t102115: F, t116414: F, t116417: F, t116420: F, t116423: F, t116427: F, t116431: F, t116435: F, t116429: F, t93351: F) -> (F, F, F, F) {
    let t116437 = t116436 / 18.0;
    let t116438 = t29706 * t379;
    let t116440 = t5674 * t22958 * t116438;
    let t116442 = -4.0 / 3.0 * t116414 + t116417 + 2.0 * t116420 - 2.0 / 3.0 * t116423 + 2.0 / 9.0 * t116427 - 4.0 / 9.0 * t116431 - t116435 + t102114 + t102115 + t116437 - 2.0 / 3.0 * t116440;
    let t116444 = t5674 * t93351 * t116429;
    (t116438, t116440, t116442, t116444)
}
