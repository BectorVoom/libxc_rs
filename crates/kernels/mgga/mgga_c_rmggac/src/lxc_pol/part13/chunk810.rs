//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 810/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk810<F: Float>(t7414: F, t8616: F, t2392: F, t833: F, t262: F, t7204: F, t5058: F, t511: F, t7284: F, t34975: F, t34976: F, t571: F, t7455: F, t39850: F, t7229: F, t4550: F, t495: F, t8440: F) -> (F, F, F, F, F, F, F) {
    let t40125 = t7414 * t8616;
    let t40134 = t2392 * t833;
    let t40135 = t262 * t40134;
    let t40136 = t7204 * t40135;
    let t40138 = t5058 * t511;
    let t40139 = t40138 * t7284;
    let t40143 = t34975 * t34976 * t571 * t7455;
    let t40145 = t7229 * t39850;
    let t40149 = t40145 * t34976 * t8440 * t4550 * t495;
    (t40125, t40134, t40135, t40136, t40139, t40143, t40149)
}
