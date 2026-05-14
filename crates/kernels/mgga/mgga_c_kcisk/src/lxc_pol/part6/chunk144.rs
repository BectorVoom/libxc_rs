//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 144/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk144<F: Float>(t338: F, t41: F, t382: F, t525: F, t79: F, t20: F, t469: F) -> (F, F, F, F) {
    let t526 = t338 * t41;
    let t529 = 10.0 / 9.0 * t525 * t526 * t382;
    let t530 = t529 < -0.66725e-1;
    let t532 = piecewise3(t530, 0.0, 0.66725e-1 + t529);
    let t533 = t79 * t532;
    let t534 = t469 * t20;
    let t535 = t533 * t534;
    (t526, t533, t534, t535)
}
