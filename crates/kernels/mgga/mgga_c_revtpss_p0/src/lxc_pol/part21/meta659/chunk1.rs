//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2452/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2452<F: Float>(t1014: F, t11150: F, t1003: F, t11735: F, t221: F, t345: F, t346: F, t624: F, t1007: F, t11738: F, t3080: F, t3083: F) -> (F, F, F, F, F) {
    let t42731 = t1014 * t11150;
    let t42740 = t1003 * t11735;
    let t42745 = F::new(5.0) / F::new(486.0) * t345 * t221 * t624 * t346;
    let t42754 = t11738 * t1007;
    let t42756 = t3083 * t3080;
    (t42731, t42740, t42745, t42754, t42756)
}
