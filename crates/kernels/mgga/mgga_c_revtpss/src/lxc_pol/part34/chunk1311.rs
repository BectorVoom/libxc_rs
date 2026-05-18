//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1311/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1311<F: Float>(t4173: F, t5826: F, t1493: F, t5872: F, t77: F, t22742: F, t84: F, t5825: F, t22672: F, t603: F, t108753: F, t108757: F, t1928: F, t28127: F, t28138: F, t29526: F, t29548: F, t29554: F, t6958: F, t7702: F, t7706: F, t7716: F, t7720: F) -> F {
    let t114296 = t4173 * t5826;
    let t114301 = t77 * t1493 * t5872;
    let t114305 = t77 * t84 * t22742;
    let t114311 = t77 * t84 * t5825;
    let t114313 = t603 * t22672;
    let t114320 = F::new(5.0) / F::new(2.0) * t28138 * t29548 + t114296 * t1928 + F::new(5.0) / F::new(2.0) * t28127 * t29548 + F::new(5.0) / F::new(2.0) * t6958 * t114301 + F::new(5.0) / F::new(6.0) * t6958 * t114305 - F::new(5.0) * t108753 * t7706 + t108757 * t114311 + t114313 * t1928 / F::new(3.0) + t29554 * t7716 + t29554 * t7720 - t7702 * t29526 / F::new(2.0);
    t114320
}
