//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1357/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1357<F: Float>(t12046: F, t15905: F, t994: F, t1014: F, t11150: F, t221: F, t345: F, t346: F, t624: F, t1065: F, t215: F, t373: F, t675: F) -> (F, F, F, F, F) {
    let t42690 = t994 * t12046 * t15905;
    let t42731 = t1014 * t11150;
    let t42745 = F::new(5.0) / F::new(486.0) * t345 * t221 * t624 * t346;
    let t42778 = t215 * t1065;
    let t42792 = t675 * t373;
    (t42690, t42731, t42745, t42778, t42792)
}
