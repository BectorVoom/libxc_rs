//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 894/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk894<F: Float>(t7343: F, t7839: F, t374: F, t7852: F, t7728: F, t3378: F, t7646: F) -> (F, F, F, F) {
    let t30624 = t7839 * t7343;
    let t30638 = t7852 * t374;
    let t30640 = t7839 * t7728;
    let t30644 = t3378 * t7646;
    (t30624, t30638, t30640, t30644)
}
