//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 511/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk511<F: Float>(t448: F, t999: F, t535: F, t988: F, t2274: F, t2278: F, t2283: F, t2285: F, t471: F, t64: F, t984: F) -> (F, F, F, F) {
    let t2738 = t999 * t448;
    let t2741 = t535 * t988;
    let t2748 = -21.0 / 128.0 * t2274 + 21.0 / 4096.0 * t2278 - 7.0 / 4096.0 * t2283 + 7.0 / 128.0 * t2285;
    let t2754 = t2748 * t471 - 4.0 / 3.0 * t984 * t64 - 7.0 / 128.0 * t2274 + 7.0 / 384.0 * t2285;
    (t2738, t2741, t2748, t2754)
}
