//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 190/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk190<F: Float>(t336: F, t579: F, t578: F, t137: F, t368: F, t121: F, t159: F) -> (F, F, F, F, F) {
    let t580 = t336 * t579;
    let t581 = t578 * t580;
    let t584 = t336 * t368 * t137;
    let t585 = t578 * t584;
    let t587 = t159 * t121;
    (t580, t581, t584, t585, t587)
}
