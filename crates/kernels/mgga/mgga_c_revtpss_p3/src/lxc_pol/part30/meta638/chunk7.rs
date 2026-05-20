//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2215/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2215<F: Float>(t25163: F, t8143: F, t101226: F, t2122: F, t101200: F, t101230: F, t25162: F, t26783: F, t26786: F, t26792: F, t26795: F, t28119: F, t28147: F, t28154: F, t29380: F, t7576: F, t7579: F, t7709: F, t92565: F, t96760: F, t96765: F, t96824: F) -> F {
    let t104314 = t8143 * t25163;
    let t104317 = t2122 * t101226;
    let t104330 = F::new(2.0) / F::new(3.0) * t28119 * t7576 + F::new(2.0) / F::new(3.0) * t28119 * t7579 + t7709 * t26783 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t7709 * t26786 - F::new(10.0) / F::new(3.0) * t28154 * t96760 - F::new(10.0) / F::new(3.0) * t25162 * t104314 - F::new(10.0) / F::new(3.0) * t25162 * t104317 - F::new(10.0) / F::new(3.0) * t101230 * t26795 - F::new(10.0) * t96824 * t28147 - F::new(10.0) / F::new(3.0) * t92565 * t29380 - F::new(5.0) / F::new(3.0) * t28154 * t96765 - F::new(10.0) * t26792 * t101200;
    t104330
}
