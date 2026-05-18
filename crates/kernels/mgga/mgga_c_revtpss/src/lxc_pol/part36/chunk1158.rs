//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1158/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1158<F: Float>(t5378: F, t7624: F, t1785: F, t7623: F, t3670: F, t2133: F, t816: F, t1224: F, t65: F, t3698: F, t1234: F, t8184: F) -> (F, F, F, F, F, F, F) {
    let t29034 = t7624 * t5378;
    let t29037 = t1785 * t7623;
    let t29040 = t3670 * t7623;
    let t29047 = t2133 * t816;
    let t29048 = t65 * t1224;
    let t29054 = t65 * t3698;
    let t29062 = t1234 * t8184;
    (t29034, t29037, t29040, t29047, t29048, t29054, t29062)
}
