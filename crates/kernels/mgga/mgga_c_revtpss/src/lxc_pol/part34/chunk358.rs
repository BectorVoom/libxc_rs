//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 358/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk358<F: Float>(t1076: F, t1647: F, t1652: F, t1680: F, t1696: F, t342: F, t386: F, t995: F) -> F {
    let t1699 = F::new(0.65854491829355115987e0) * t1647 * t386 - F::new(0.65854491829355115987e0) * t995 * t1652 + F::new(0.65854491829355115987e0) * t342 * t1680 - F::new(0.65854491829355115987e0) * t1076 * t1696;
    t1699
}
