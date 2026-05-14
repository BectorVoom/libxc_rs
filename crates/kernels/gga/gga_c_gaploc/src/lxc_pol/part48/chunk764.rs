//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 764/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk764<F: Float>(t1843: F, t35500: F, t7064: F, t35550: F, t5539: F, t13545: F, t7137: F, t13495: F, t7129: F, t2508: F, t2717: F, t3616: F, t11588: F, t954: F, t10667: F, t2958: F) -> (F, F, F, F, F, F, F) {
    let t45051 = t7064 * t1843 * t35500;
    let t45052 = 0.32043859292259267849e-3 * t45051;
    let t45054 = t7064 * t5539 * t35550;
    let t45057 = 0.71778244814660759981e-1 * t7137 * t13545;
    let t45059 = 0.76905262301422242837e-2 * t7129 * t13495;
    let t45062 = 0.76905262301422242837e-2 * t2508 * t2717 * t3616;
    let t45065 = 0.76905262301422242837e-2 * t2508 * t954 * t11588;
    let t45066 = t2958 * t10667;
    (t45052, t45054, t45057, t45059, t45062, t45065, t45066)
}
