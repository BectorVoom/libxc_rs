//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1122/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1122<F: Float>(t93172: F, t93173: F, t93170: F, t25305: F, t92894: F, t786: F, t92889: F, t7060: F, t2434: F, t837: F, t25377: F, t25431: F, t213: F, t25286: F, t251: F, t25304: F) -> (F, F, F, F, F, F, F, F) {
    let t93174 = t93172 * t93173;
    let t93175 = t93170 * t93174;
    let t93177 = t25305 * t92894;
    let t93179 = t786 * t92889;
    let t93180 = t93179 * t7060;
    let t93182 = t2434 * t837;
    let t93183 = t25377 * t93182;
    let t93184 = t25431 * t93183;
    let t93186 = t213 * t25286;
    let t93189 = t25304 * t251;
    (t93174, t93175, t93177, t93180, t93183, t93184, t93186, t93189)
}
