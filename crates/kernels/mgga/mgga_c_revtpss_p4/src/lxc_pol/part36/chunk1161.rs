//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1161/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1161<F: Float>(t29135: F, t7642: F, t2148: F, t11239: F, t1276: F, t3596: F, t2149: F, t1243: F, t460: F, t8190: F, t1209: F, t1770: F, t2142: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29136 = t7642 * t29135;
    let t29141 = t2148 * t29135;
    let t29192 = t11239 * t1276;
    let t29193 = t29192 * t3596;
    let t29194 = t2149 * t29193;
    let t29199 = t29192 * t1243;
    let t29200 = t2149 * t29199;
    let t29207 = t460 * t8190;
    let t29220 = t1209 * t8190;
    let t29227 = t1770 * t2142;
    (t29136, t29141, t29193, t29194, t29199, t29200, t29207, t29220, t29227)
}
