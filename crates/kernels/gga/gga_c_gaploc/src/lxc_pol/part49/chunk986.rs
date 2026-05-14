//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 986/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk986<F: Float>(t43274: F, t43275: F, t43278: F, t43282: F, t43283: F, t43286: F, t43288: F, t43289: F, t43290: F, t47737: F, t47740: F, t2508: F, t2541: F, t39022: F, t13918: F, t7129: F) -> (F, F, F) {
    let t47744 = -0.23071578690426672851e-1 * t47737 - 0.23071578690426672851e-1 * t47740 - t43274 + t43275 + 0.76905262301422242837e-2 * t43278 - t43282 - t43283 - t43286 + t43288 - t43289 - 0.42725145723012357132e-3 * t43290;
    let t47749 = t2508 * t2541 * t39022;
    let t47752 = t7129 * t13918;
    (t47744, t47749, t47752)
}
