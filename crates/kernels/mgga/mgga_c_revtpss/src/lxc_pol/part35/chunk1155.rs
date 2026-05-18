//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1155/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1155<F: Float>(t18531: F, t25245: F, t18432: F, t93025: F, t18440: F, t25227: F, t2661: F, t18348: F, t1945: F, t807: F, t25266: F, t6019: F) -> (F, F, F, F, F) {
    let t106048 = t25245 * t18531;
    let t106050 = t93025 * t18432;
    let t106053 = t2661 * t25227 * t18440;
    let t106061 = t807 * t1945 * t18348;
    let t106063 = t25266 * t6019;
    (t106048, t106050, t106053, t106061, t106063)
}
