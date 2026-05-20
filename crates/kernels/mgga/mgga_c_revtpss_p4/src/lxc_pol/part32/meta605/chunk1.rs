//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1944/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1944<F: Float>(t18657: F, t1955: F, t1579: F, t231: F, t4423: F, t25207: F, t77441: F, t1544: F, t580: F, t98646: F, t18435: F, t27159: F) -> (F, F, F, F, F) {
    let t106404 = t1955 * t18657;
    let t106410 = t1579 * t4423 * t231;
    let t106490 = t25207 * t77441;
    let t106494 = t98646 * t580 * t1544;
    let t106498 = t27159 * t18435;
    (t106404, t106410, t106490, t106494, t106498)
}
