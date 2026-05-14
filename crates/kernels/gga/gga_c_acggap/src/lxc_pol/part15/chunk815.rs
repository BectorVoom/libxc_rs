//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 815/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk815<F: Float>(t145: F, t301: F, t30598: F, t721: F, t174: F, t372: F, t7859: F, t30105: F, t7348: F, t374: F, t7852: F, t3378: F, t7646: F) -> (F, F, F, F, F) {
    let t30601 = t30598 * t145 * t301 * t721;
    let t30605 = t7859 * t174 * t372 * t721;
    let t30613 = t30105 * t7348;
    let t30638 = t7852 * t374;
    let t30644 = t3378 * t7646;
    (t30601, t30605, t30613, t30638, t30644)
}
