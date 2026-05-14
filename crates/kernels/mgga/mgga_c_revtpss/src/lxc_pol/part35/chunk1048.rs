//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1048/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1048<F: Float>(t23281: F, t7045: F, t23257: F, t25262: F, t23285: F, t7038: F, t23342: F, t23289: F, t23253: F, t93062: F, t1579: F, t231: F, t6016: F, t5977: F, t2723: F, t1955: F, t23359: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t113226 = t7045 * t23281;
    let t113228 = t25262 * t23257;
    let t113230 = t7038 * t23285;
    let t113232 = t7045 * t23342;
    let t113235 = t7038 * t23289;
    let t113237 = t93062 * t23253;
    let t113269 = t1579 * t6016 * t231;
    let t113285 = t1579 * t5977;
    let t113286 = t113285 * t231;
    let t113295 = t113285 * t2723;
    let t113373 = t1955 * t23359;
    (t113226, t113228, t113230, t113232, t113235, t113237, t113269, t113286, t113295, t113373)
}
