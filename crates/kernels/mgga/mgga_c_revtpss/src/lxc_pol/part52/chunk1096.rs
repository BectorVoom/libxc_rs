//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1096/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1096<F: Float>(t128289: F, t128291: F, t128293: F, t128294: F, t128295: F, t128301: F, t128303: F, t128305: F, t128306: F, t2007: F, t25805: F, t28025: F, t28050: F, t28683: F, t28750: F, t651: F, t671: F, t6985: F, t7359: F, t7988: F) -> (F,) {
    let t128307 = -t2007 * t28683 * t651 - t128291 * t671 - t25805 * t7988 - t28025 * t7988 - t28050 * t7359 - t28750 * t6985 - t128289 - t128293 - t128294 - t128295 - t128301 - t128303 - t128305 - t128306;
    (t128307,)
}
