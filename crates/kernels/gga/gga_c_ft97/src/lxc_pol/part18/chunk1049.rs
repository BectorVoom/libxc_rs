//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1049/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1049<F: Float>(t34433: F, t420: F, t1008: F, t71: F, t1013: F, t388: F, t7866: F, t2: F, t7242: F, t369: F, t7954: F, t23: F, t32075: F, t1609: F, t7905: F, t1610: F, t1613: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34434 = t420 * t34433;
    let t34871 = t71 * t1008;
    let t34876 = t71 * t1013;
    let t36368 = t388 * t7866;
    let t36452 = t7242 * t2;
    let t37305 = t7954 * t369;
    let t37429 = t23 * t32075;
    let t37452 = t1609 * t7905;
    let t37481 = t1613 * t1610 * t1609;
    (t34434, t34871, t34876, t36368, t36452, t37305, t37429, t37452, t37481)
}
