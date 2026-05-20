//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2220/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2220<F: Float>(t104409: F, t104427: F, t13429: F, t1518: F, t18153: F, t2127: F, t2163: F, t2371: F, t27056: F, t29456: F, t4254: F, t569: F, t651: F, t8233: F, t97661: F, t97663: F, t97666: F, t98421: F, t98426: F, t98428: F, t98430: F, t98432: F, t98439: F, t98440: F, t98442: F, t98449: F, t98452: F) -> F {
    let t104433 = t97661 - F::new(4.0) * t4254 * t29456 - F::new(2.0) * t651 * t27056 * t1518 - t97663 - t97666 + t98421 - F::new(2.0) * t13429 * t2163 - t98426 - t98428 - t98430 - t98432 - t2127 * t18153 + (t104409 + t104427) * t569 - t98439 + t98440 - t98442 - F::new(2.0) * t651 * t8233 * t2371 + t98449 - t98452;
    t104433
}
