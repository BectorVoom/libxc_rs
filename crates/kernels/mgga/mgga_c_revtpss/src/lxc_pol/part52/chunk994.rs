//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 994/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk994<F: Float>(t4146: F, t550: F, t9794: F, t243: F, t2246: F, t4171: F, t10308: F, t1466: F, t1925: F, t606: F, t1962: F, t41154: F, t2411: F, t605: F, t2453: F, t251: F) -> (F, F, F, F, F, F, F, F, F) {
    let t47671 = t4146 * t4146;
    let t47672 = 1.0 / t47671;
    let t49068 = t9794 * t550;
    let t51076 = t9794 * t243;
    let t60221 = t4171 * t2246;
    let t60224 = t1466 * t10308;
    let t92669 = t606 * t1925;
    let t92742 = t1962 * t41154;
    let t92790 = t2411 * t605;
    let t93169 = t2453 * t251;
    (t47672, t49068, t51076, t60221, t60224, t92669, t92742, t92790, t93169)
}
