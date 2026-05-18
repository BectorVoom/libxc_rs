//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1179/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1179<F: Float>(t1294: F, t5457: F, t11239: F, t1276: F, t3596: F, t2149: F, t1243: F, t3555: F, t7635: F, t2126: F, t670: F, t4147: F, t7311: F) -> (F, F, F, F, F, F, F, F) {
    let t29166 = t5457 * t1294;
    let t29192 = t11239 * t1276;
    let t29193 = t29192 * t3596;
    let t29194 = t2149 * t29193;
    let t29199 = t29192 * t1243;
    let t29200 = t2149 * t29199;
    let t29204 = t3555 * t7635;
    let t29432 = t2126 * t670;
    let t32113 = t4147 * t7311;
    (t29166, t29193, t29194, t29199, t29200, t29204, t29432, t32113)
}
