//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1963/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1963<F: Float>(t1203: F, t5457: F, t29158: F, t5458: F, t1294: F, t2142: F, t5215: F, t7637: F, t1828: F, t7627: F, t7652: F, t225: F, t29109: F, t494: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t29159 = t5457 * t1203;
    let t29160 = t29158 * t29159;
    let t29163 = t29158 * t5458;
    let t29166 = t5457 * t1294;
    let t29167 = t29158 * t29166;
    let t29174 = t2142 * t5215;
    let t29175 = t7637 * t29174;
    let t29178 = t7627 * t1828;
    let t29179 = t7652 * t29178;
    let t29183 = t29109 * t225 * t494;
    (t29159, t29160, t29163, t29166, t29167, t29174, t29175, t29178, t29179, t29183)
}
