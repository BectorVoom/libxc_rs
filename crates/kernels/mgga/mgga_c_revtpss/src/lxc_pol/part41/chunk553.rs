//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 553/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk553<F: Float>(t2629: F, t2630: F, t73: F, t853: F, t820: F, t843: F, t849: F) -> (F, F, F) {
    let t2632 = 0.10843581300301739842e-1 * t2629 * t2630;
    let t2638 = t73 * t853;
    let t2652 = t820 * t849 * t843;
    (t2632, t2638, t2652)
}
