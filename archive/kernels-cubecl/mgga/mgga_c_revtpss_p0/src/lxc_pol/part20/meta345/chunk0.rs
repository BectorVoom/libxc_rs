//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1272/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1272<F: Float>(t1263: F, t3362: F, t12256: F, t13099: F, t1224: F, t140: F, t1260: F, t12966: F, t12987: F, t15687: F, t3623: F, t3782: F) -> (F, F, F, F, F, F, F) {
    let t17202 = t1263 * t3362;
    let t17235 = t13099 * t12256;
    let t17240 = t140 * t1224;
    let t17261 = t12966 * t1260;
    let t17344 = t12987 * t1260;
    let t17350 = t3623 * t15687;
    let t17351 = t3782 * t17350;
    (t17202, t17235, t17240, t17261, t17344, t17350, t17351)
}
