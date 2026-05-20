//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2249/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2249<F: Float>(t28182: F, t7235: F, t13392: F, t603: F, t13396: F, t13405: F, t1928: F, t25140: F, t25143: F, t25147: F, t28112: F, t28116: F, t28119: F, t6974: F, t6978: F, t7709: F) -> (F, F) {
    let t101124 = F::new(2.0) * t7235 * t28182;
    let t101129 = t603 * t13392;
    let t101132 = t603 * t13396;
    let t101139 = t603 * t13405;
    let t101152 = F::new(2.0) / F::new(3.0) * t28112 * t6974 + F::new(2.0) / F::new(3.0) * t28112 * t6978 + t101129 * t1928 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t101132 * t1928 + F::new(2.0) / F::new(3.0) * t28116 * t6974 + F::new(2.0) / F::new(3.0) * t28116 * t6978 + t101139 * t1928 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t28119 * t6974 + F::new(2.0) / F::new(3.0) * t28119 * t6978 + t7709 * t25140 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t7709 * t25143 + t7709 * t25147 / F::new(3.0);
    (t101124, t101152)
}
