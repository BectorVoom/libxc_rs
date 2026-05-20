//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 478/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk478<F: Float>(t118: F, t1310: F, t1315: F, t1453: F, t508: F, t511: F, t569: F, t649: F, t651: F, t671: F, t3: F, t571: F) -> (F, F, F) {
    let t1455 = -t118 * t1310 + t1315 * t569 + t1453 * t511 - t508 * t649 - F::new(2.0) * t651 * t671;
    let t1456 = t3 * t1455;
    let t1458 = t3 * t571;
    (t1455, t1456, t1458)
}
