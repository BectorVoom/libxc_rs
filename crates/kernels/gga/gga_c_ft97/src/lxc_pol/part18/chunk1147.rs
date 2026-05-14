//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1147/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1147<F: Float>(t1871: F, t22952: F, t3103: F, t473: F, t5675: F, t1588: F, t8411: F, t965: F, t11982: F, t23031: F, t1564: F, t446: F, t11437: F, t92196: F, t7793: F, t22986: F) -> (F, F, F, F, F, F, F) {
    let t100219 = t22952 * t1871 * t5675 * t3103 * t473;
    let t100224 = t22952 * t8411 * t5675 * t965 * t1588;
    let t100226 = t23031 * t11982;
    let t100228 = t446 * t1564 * t100226;
    let t100230 = t92196 * t11437;
    let t100232 = t446 * t7793 * t100230;
    let t100234 = t22986 * t11982;
    (t100219, t100224, t100226, t100228, t100230, t100232, t100234)
}
