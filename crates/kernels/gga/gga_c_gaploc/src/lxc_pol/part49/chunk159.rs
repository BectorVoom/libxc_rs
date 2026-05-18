//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 159/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk159<F: Float>(t667: F, t758: F, t10: F, t107: F, t183: F, t266: F, t305: F, t306: F, t677: F, t749: F, t753: F, t755: F) -> F {
    let t759 = t758 * t667;
    let t768 = F::new(0.58998125e-2) * t749 * t306 - F::new(0.11799625e-1) * t753 * t755 - F::new(0.58998125e-2) * t305 * t759 - F::new(0.14341111111111111111e-1) * t107 * t10 * t266 - F::new(0.21511666666666666667e-1) * t107 * t183 * t677;
    t768
}
