//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2213/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2213<F: Float>(t101129: F, t101132: F, t101190: F, t101193: F, t101350: F, t2123: F, t25102: F, t25120: F, t28112: F, t28116: F, t29372: F, t6963: F, t7566: F, t7576: F, t7579: F, t8147: F) -> F {
    let t104274 = F::new(2.0) / F::new(3.0) * t101190 * t2123 + F::new(2.0) / F::new(3.0) * t101193 * t2123 + F::new(2.0) / F::new(3.0) * t28112 * t7576 + F::new(2.0) / F::new(3.0) * t28112 * t7579 + t101129 * t2123 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t101132 * t2123 + F::new(2.0) / F::new(3.0) * t28116 * t7576 + F::new(2.0) / F::new(3.0) * t28116 * t7579 + F::new(5.0) / F::new(6.0) * t7566 * t101350 + F::new(2.0) / F::new(3.0) * t25102 * t8147 + t25120 * t8147 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t6963 * t29372;
    t104274
}
