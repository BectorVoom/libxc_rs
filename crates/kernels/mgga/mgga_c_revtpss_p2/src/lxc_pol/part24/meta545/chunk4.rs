//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1616/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1616<F: Float>(t2723: F, t87399: F, t39419: F, t39422: F, t39429: F, t39432: F, t87262: F, t87263: F, t87265: F, t87267: F, t87268: F, t87296: F, t87298: F) -> (F, F) {
    let t87629 = t87399 * t2723;
    let t87634 = t87262 + t87263 + t87265 - t39419 - t39422 + t87267 - t87268 + t87296 + t87298 - t39429 - t39432;
    (t87629, t87634)
}
