//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2834/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2834<F: Float>(t1003: F, t11735: F, t221: F, t345: F, t346: F, t624: F, t3080: F, t3083: F, t11858: F, t16048: F, t1065: F, t215: F) -> (F, F, F, F, F) {
    let t42740 = t1003 * t11735;
    let t42745 = F::cast_from(5.0_f64) / F::cast_from(486.0_f64) * t345 * t221 * t624 * t346;
    let t42756 = t3083 * t3080;
    let t42765 = t11858 * t16048;
    let t42778 = t215 * t1065;
    (t42740, t42745, t42756, t42765, t42778)
}
