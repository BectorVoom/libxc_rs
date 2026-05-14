//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 698/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk698<F: Float>(t8562: F, t8564: F, t8758: F, t8917: F, t118: F, t2127: F, t2163: F, t508: F, t569: F, t8456: F, t8463: F, t8597: F, t8601: F, t8743: F, t8750: F, t8765: F, t8964: F) -> (F, F) {
    let t8967 = t8917 + 4.0 * t8758 + t8562 + t8564;
    let t8970 = -t118 * t8964 - 2.0 * t2127 * t2163 - t508 * t8917 + t569 * t8967 - t8456 - t8463 + t8597 - t8601 - 4.0 * t8743 - 4.0 * t8750 + 2.0 * t8765;
    (t8967, t8970)
}
