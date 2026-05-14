//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1075/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1075<F: Float>(t102981: F, t102994: F, t103001: F, t103009: F, t110236: F, t110245: F, t110247: F, t110276: F, t95542: F, t95548: F, t95567: F, t95569: F, t95607: F, t95632: F, t2061: F, t23167: F) -> (F, F) {
    let t115493 = 0.86736281882051994623e-1 * t110236 - 0.16463622957338778996e-1 * t110245 - t95542 - 0.15421710918628844643e0 * t110247 - t95548 + t95567 + t95569 - 0.10281140612419229763e-1 * t102981 - 0.23132566377943266966e0 * t110276 + 0.28912093960683998208e-1 * t102994 - t95607 - 0.51405703062096148814e-2 * t103001 + 0.13709901006661042888e-1 * t103009 + t95632;
    let t115499 = t2061 * t23167;
    (t115493, t115499)
}
