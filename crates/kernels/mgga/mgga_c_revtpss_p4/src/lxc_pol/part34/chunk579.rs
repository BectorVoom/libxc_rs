//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 579/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk579<F: Float>(t2611: F, t6002: F, t2498: F, t2518: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t2610: F, t2621: F, t2628: F, t2632: F, t5924: F, t5925: F, t5927: F, t5943: F, t5945: F, t5947: F, t5948: F, t6001: F) -> (F, F) {
    let t6004 = F::new(12.0) * t2611 * t6002;
    let t6005 = -t2498 - t2518 - t2522 + t5947 + t2610 + t2579 + t2587 + t6001 - t2562 + t5925 - t2569 + t2621 + t2628 + t2632 + t6004 + t5943 + t5945 - t5924 - t5948 + t5927;
    (t6004, t6005)
}
