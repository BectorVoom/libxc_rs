//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2266/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2266<F: Float>(t104094: F, t105759: F, t105762: F, t105775: F, t1456: F, t1458: F, t1464: F, t18178: F, t1914: F, t1921: F, t2172: F, t27090: F, t27110: F, t29469: F, t29490: F, t3: F, t4154: F, t4168: F, t575: F, t5790: F, t5808: F, t7691: F, t7700: F, t8241: F, t8249: F, t96690: F) -> F {
    let t105789 = t18178 * t2172 + F::new(2.0) * t96690 + t1914 * t27110 + F::new(2.0) * t5790 * t7700 + t104094 + t1458 * (t105762 + t105775) + F::new(2.0) * t1456 * t29490 + t4154 * t8249 + t3 * t105759 * t575 + t27090 * t1921 + F::new(2.0) * t7691 * t5808 + F::new(2.0) * t29469 * t1464 + t8241 * t4168;
    t105789
}
