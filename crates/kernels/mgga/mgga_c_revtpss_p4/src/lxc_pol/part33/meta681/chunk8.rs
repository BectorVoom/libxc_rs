//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2229/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2229<F: Float>(t104115: F, t109204: F, t109222: F, t109224: F, t109226: F, t109228: F, t109230: F, t111696: F, t111734: F, t1518: F, t21881: F, t27060: F, t29427: F, t29432: F, t34446: F, t4292: F, t5920: F, t670: F, t7586: F) -> F {
    let t111788 = F::new(4.0) * t104115 * t1518 + F::new(2.0) * t111696 * t670 + F::new(4.0) * t111734 * t1518 + F::new(2.0) * t21881 * t7586 + F::new(2.0) * t27060 * t5920 + F::new(4.0) * t29427 * t4292 + F::new(2.0) * t29432 * t5920 + F::new(4.0) * t34446 * t4292 + t109204 + t109222 + t109224 + t109226 + t109228 + t109230;
    t111788
}
