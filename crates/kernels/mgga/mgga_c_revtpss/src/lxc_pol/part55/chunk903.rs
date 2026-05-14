//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 903/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk903<F: Float>(t2163: F, t4292: F, t670: F, t8233: F, t1519: F, t1911: F, t2165: F, t28183: F, t28186: F, t28188: F, t28190: F, t28192: F, t28193: F, t28201: F, t28202: F, t29432: F, t4248: F, t4257: F, t5787: F, t651: F, t7586: F, t7591: F, t7687: F) -> (F, F, F) {
    let t29456 = t2163 * t4292;
    let t29459 = t8233 * t670;
    let t29466 = -2.0 * t1519 * t29432 + t1911 * t7687 + t2165 * t5787 - 2.0 * t29456 * t651 - 2.0 * t29459 * t651 - 2.0 * t4248 * t7591 - 2.0 * t4257 * t7586 - t28183 + t28186 - t28188 - t28190 + t28192 - t28193 + t28201 - t28202;
    (t29456, t29459, t29466)
}
