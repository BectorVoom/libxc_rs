//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1328/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1328<F: Float>(t105590: F, t105596: F, t105599: F, t105603: F, t105608: F, t105611: F, t105614: F, t105618: F, t105620: F, t105623: F, t96077: F, t96078: F, t26768: F, t28: F, t586: F, t5890: F, t590: F) -> (F, F) {
    let t105625 = 2.0 / 3.0 * t105590 + 15.0 / 16.0 * t105596 + t105599 - t96077 - t96078 + 2.0 * t105603 + t105608 / 2.0 - 4.0 / 9.0 * t105611 - 4.0 / 3.0 * t105614 + t105618 - 2.0 / 3.0 * t105620 - 4.0 / 3.0 * t105623;
    let t105629 = t5890 * t28 * t586 * t26768 * t590;
    (t105625, t105629)
}
