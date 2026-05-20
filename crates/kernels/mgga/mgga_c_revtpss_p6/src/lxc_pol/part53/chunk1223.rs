//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1223/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1223<F: Float>(t34399: F, t7313: F, t28166: F, t8763: F, t28168: F, t125537: F, t125539: F, t125541: F, t125543: F, t125545: F, t125547: F, t125550: F, t125552: F, t125554: F, t2163: F, t28160: F, t7683: F, t7725: F) -> F {
    let t129376 = t34399 * t7313;
    let t129377 = t8763 * t28166;
    let t129378 = t129377 * t28168;
    let t129391 = -t2163 * t28160 - t7683 * t7725 - F::new(2.0) * t125537 - F::new(2.0) * t125539 - F::new(2.0) * t125541 - F::new(2.0) * t125543 - F::new(2.0) * t125545 - F::new(2.0) * t125547 - F::new(2.0) * t125550 - F::new(2.0) * t125552 - F::new(2.0) * t125554 + t129376 + F::new(6.0) * t129378;
    t129391
}
