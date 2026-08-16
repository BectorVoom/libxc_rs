//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1177/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1177<F: Float>(t20555: F, t7450: F, t8915: F, t2030: F, t20559: F, t8919: F, t301: F, t31146: F, t4256: F, t9529: F, t1891: F, t7614: F) -> (F, F, F, F) {
    let t40374 = t7450 * t20555 * t8915;
    let t40377 = t2030 * t20559 * t8919;
    let t40381 = t31146 * t4256 * t9529 * t301;
    let t40385 = t7614 * t1891;
    (t40374, t40377, t40381, t40385)
}
