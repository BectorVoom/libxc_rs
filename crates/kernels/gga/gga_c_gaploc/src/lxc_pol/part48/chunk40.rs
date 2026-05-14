//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 40/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk40<F: Float>(t135: F, t139: F, t35: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t34 = 1.0 <= zeta_threshold;
    let t140 = t135 * t139;
    let t141 = t35 * t35;
    let t142 = piecewise3(t34, t141, 1.0);
    let t143 = t142 * t142;
    let t145 = 1.0 / t143 / t142;
    (t140, t141, t142, t143, t145)
}
