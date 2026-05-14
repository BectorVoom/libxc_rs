//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 560/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk560<F: Float>(t9097: F, t9100: F, t9108: F, t9111: F, t9113: F, t9115: F, t10209: F, t3526: F, t471: F, t64: F) -> (F, F) {
    let t11210 = -21.0 / 128.0 * t9097 + 147.0 / 4096.0 * t9100 - 63.0 / 262144.0 * t9108 + 21.0 / 262144.0 * t9111 - 49.0 / 4096.0 * t9113 + 7.0 / 128.0 * t9115;
    let t11218 = t11210 * t471 - 4.0 / 3.0 * t3526 * t64 + t10209 - 7.0 / 128.0 * t9097 + 21.0 / 4096.0 * t9100 - 7.0 / 4096.0 * t9113 + 7.0 / 384.0 * t9115;
    (t11210, t11218)
}
