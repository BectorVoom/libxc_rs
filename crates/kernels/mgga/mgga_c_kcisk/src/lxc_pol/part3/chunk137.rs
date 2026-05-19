//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 137/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk137<F: Float>(t469: F, t470: F, t468: F, t415: F, t338: F, t412: F, t196: F) -> (F, F, F, F, F) {
    let t471 = t469 * t470;
    let t472 = t468 * t471;
    let t473 = t415 * t472;
    let t475 = t338 * t412 + F::cast_from(0.24872916666666666666e-2_f64) * t473;
    let t476 = t338 * t196;
    (t471, t472, t473, t475, t476)
}
