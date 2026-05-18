//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 943/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk943<F: Float>(t8422: F, t8577: F, t8427: F, t8432: F, t8437: F, t40661: F, t8443: F, t2001: F, t2281: F, t326: F, t558: F, t7720: F) -> (F, F, F, F, F, F) {
    let t45670 = t8577 * t8422;
    let t45672 = t8577 * t8427;
    let t45674 = t8577 * t8432;
    let t45676 = t8577 * t8437;
    let t45678 = t40661 * t8443;
    let t45685 = t2001 * t326 * t2281 * t558;
    let t45686 = t7720 * t45685;
    (t45670, t45672, t45674, t45676, t45678, t45686)
}
