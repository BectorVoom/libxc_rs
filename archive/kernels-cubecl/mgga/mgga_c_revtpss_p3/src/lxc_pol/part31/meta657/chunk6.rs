//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2220/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2220<F: Float>(t108710: F, t1937: F, t108117: F, t108120: F, t108129: F, t108681: F, t108685: F, t108687: F, t108691: F, t108693: F, t1453: F, t1502: F, t1519: F, t2007: F, t21881: F, t21882: F, t27830: F, t28030: F, t28050: F, t29986: F, t30150: F, t4246: F, t4248: F, t4257: F, t4293: F, t651: F, t670: F, t6985: F, t7883: F, t97622: F) -> F {
    let t108712 = F::cast_from(2.0_f64) * t108710 * t1937;
    let t108713 = -F::cast_from(2.0_f64) * t2007 * t21881 * t651 - F::cast_from(2.0_f64) * t29986 * t651 * t670 - F::cast_from(4.0_f64) * t108120 * t1519 + t1453 * t30150 - F::cast_from(2.0_f64) * t1502 * t27830 - F::cast_from(4.0_f64) * t1519 * t97622 - F::cast_from(2.0_f64) * t21882 * t6985 - F::cast_from(4.0_f64) * t28030 * t4257 - F::cast_from(4.0_f64) * t28030 * t4293 - F::cast_from(4.0_f64) * t28050 * t4248 - F::cast_from(2.0_f64) * t4246 * t7883 - t108117 - t108129 + t108681 - t108685 + t108687 + t108691 + t108693 - t108712;
    t108713
}
