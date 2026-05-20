//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2134/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2134<F: Float>(t1931: F, t2371: F, t13426: F, t13544: F, t1519: F, t18153: F, t18163: F, t1932: F, t2372: F, t25805: F, t27145: F, t28025: F, t28030: F, t4254: F, t4257: F, t4293: F, t6985: F, t7007: F, t7746: F, t98472: F, t98474: F, t98477: F, t98483: F, t98486: F, t98489: F, t98491: F, t98494: F, t98499: F, t98501: F) -> (F, F) {
    let t98507 = t1931 * t2371;
    let t98512 = -F::new(4.0) * t13426 * t7007 - F::new(2.0) * t13544 * t6985 - F::new(2.0) * t1519 * t98507 - t18153 * t1932 - F::new(2.0) * t18163 * t7746 - F::new(2.0) * t2372 * t28030 - F::new(4.0) * t25805 * t4293 - F::new(4.0) * t27145 * t4254 - F::new(4.0) * t28025 * t4257 - t98472 - t98474 - t98477 - t98483 - t98486 - t98489 - t98491 - t98494 - t98499 + t98501;
    (t98507, t98512)
}
