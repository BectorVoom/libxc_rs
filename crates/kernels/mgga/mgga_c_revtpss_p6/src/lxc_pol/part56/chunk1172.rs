//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1172/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1172<F: Float>(t670: F, t8916: F, t124533: F, t125531: F, t125536: F, t125558: F, t125562: F, t129354: F, t129357: F, t129366: F, t129371: F, t129376: F, t129378: F, t129395: F, t1519: F, t27060: F, t29432: F, t29444: F, t29456: F, t33346: F, t4257: F, t7586: F, t8158: F) -> (F, F) {
    let t131338 = t8916 * t670;
    let t131356 = -F::new(2.0) * t124533 * t1519 - F::new(2.0) * t131338 * t1519 - F::new(4.0) * t27060 * t8158 - F::new(4.0) * t29432 * t8158 - F::new(4.0) * t29444 * t7586 - F::new(4.0) * t29456 * t7586 - F::new(2.0) * t33346 * t4257 - t125531 + t125536 + t125558 - t125562 + F::new(4.0) * t129354 - F::new(4.0) * t129357 - F::new(6.0) * t129366 + F::new(2.0) * t129371 + F::new(2.0) * t129376 + F::new(12.0) * t129378 - F::new(4.0) * t129395;
    (t131338, t131356)
}
