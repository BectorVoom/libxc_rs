//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 580/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk580<F: Float>(t11341: F, t11381: F, t11421: F, t11451: F, t11465: F, t11499: F, t11535: F, t11553: F, t9664: F, t9666: F, t9669: F, t9672: F, t9674: F, t9676: F, t10661: F, t3611: F, t471: F, t64: F) -> (F, F, F) {
    let t11556 = t11341 + t11381 + t11421 + t11451 + t11465 + t11499 + t11535 + t11553;
    let t11568 = -21.0 / 128.0 * t9664 + 147.0 / 4096.0 * t9666 - 63.0 / 262144.0 * t9669 + 21.0 / 262144.0 * t9672 - 49.0 / 4096.0 * t9674 + 7.0 / 128.0 * t9676;
    let t11576 = t11568 * t471 - 4.0 / 3.0 * t3611 * t64 + t10661 - 7.0 / 128.0 * t9664 + 21.0 / 4096.0 * t9666 - 7.0 / 4096.0 * t9674 + 7.0 / 384.0 * t9676;
    (t11556, t11568, t11576)
}
