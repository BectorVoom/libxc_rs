//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 577/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk577<F: Float>(t2347: F, t321: F, t262: F, t8640: F, t333: F, t7198: F, t352: F, t7204: F, t1987: F, t8571: F, t5011: F, t681: F, t2085: F, t2373: F, t1679: F, t511: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8641 = t2347 * t321;
    let t8642 = t262 * t8641;
    let t8643 = t8640 * t8642;
    let t8645 = t2347 * t333;
    let t8646 = t262 * t8645;
    let t8647 = t7198 * t8646;
    let t8649 = t2347 * t352;
    let t8650 = t262 * t8649;
    let t8651 = t7204 * t8650;
    let t8653 = t8571 * t1987;
    let t8655 = t5011 * t681;
    let t8657 = t2373 * t2085;
    let t8659 = t1679 * t511;
    (t8641, t8642, t8643, t8645, t8646, t8647, t8649, t8650, t8651, t8653, t8655, t8657, t8659)
}
