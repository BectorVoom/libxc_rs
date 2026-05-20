//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2708/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2708<F: Float>(t22267: F, t4018: F, t22079: F, t5673: F, t5675: F, t1353: F, t6836: F, t828: F, t9942: F, t1868: F, t5591: F, t4012: F) -> (F, F, F, F, F, F) {
    let t22268 = t4018 * t22267;
    let t22271 = t5673 * t22079 * t5675;
    let t22274 = t6836 * t1353;
    let t22276 = t9942 * t828 * t22274;
    let t22279 = t1868 * t5591;
    let t22281 = t4012 * t828 * t22279;
    (t22268, t22271, t22274, t22276, t22279, t22281)
}
