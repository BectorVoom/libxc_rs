//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2261/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2261<F: Float>(t104416: F, t1519: F, t1911: F, t2372: F, t27060: F, t27066: F, t29427: F, t4257: F, t96706: F, t98559: F, t98562: F, t98567: F, t98569: F, t98571: F, t98574: F, t98578: F, t98581: F, t98584: F, t98590: F, t98594: F, t98597: F, t98599: F, t98601: F) -> F {
    let t105734 = -F::new(4.0) * t104416 * t1519 - F::new(2.0) * t1519 * t96706 + t1911 * t27066 - F::new(2.0) * t2372 * t29427 - F::new(4.0) * t27060 * t4257 - t98559 + t98562 + t98567 - t98569 - t98571 - t98574 + t98578 + t98581 - t98584 + t98590 + t98594 - t98597 - t98599 - t98601;
    t105734
}
