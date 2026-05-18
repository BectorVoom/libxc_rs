//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1071/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1071<F: Float>(t2060: F, t507: F, t7443: F, t7450: F, t7451: F, t7447: F, t8813: F, t8817: F, t7440: F, t8820: F, t2274: F, t30307: F) -> (F, F, F, F, F, F) {
    let t35065 = t2060 * t507 * t7443;
    let t35068 = t7450 * t507 * t7451;
    let t35070 = t7447 * t8813;
    let t35072 = t7447 * t8817;
    let t35074 = t7440 * t8820;
    let t35076 = t30307 * t2274;
    (t35065, t35068, t35070, t35072, t35074, t35076)
}
